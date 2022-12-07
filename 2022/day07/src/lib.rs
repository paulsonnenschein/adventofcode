use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, not_line_ending, u32 as parse_u32},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CdCommandParam {
    Root,
    Up,
    Down(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectoryListing {
    File(u32, String),
    Directory(String),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    LsCommand(Vec<DirectoryListing>),
    CdCommand(CdCommandParam),
}

fn cd_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ cd ")(i)?;

    let parse_root = map(tag("/"), |_| Command::CdCommand(CdCommandParam::Root));
    let parse_up = map(tag(".."), |_| Command::CdCommand(CdCommandParam::Up));
    let parse_down = map(alpha1, |str: &str| {
        Command::CdCommand(CdCommandParam::Down(str.into()))
    });
    alt((parse_root, parse_up, parse_down))(i)
}

fn ls_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tuple((tag("$ ls"), line_ending))(i)?;

    map(separated_list0(line_ending, directory_listing), |entries| {
        Command::LsCommand(entries)
    })(i)
}

fn directory_listing(i: &str) -> IResult<&str, DirectoryListing> {
    let parse_dir = map(tuple((tag("dir "), alpha1)), |(_, dir): (&str, &str)| {
        DirectoryListing::Directory(dir.into())
    });
    let parse_file = map(
        tuple((parse_u32, tag(" "), not_line_ending)),
        |(size, _, filename): (u32, &str, &str)| DirectoryListing::File(size, filename.into()),
    );
    alt((parse_dir, parse_file))(i)
}

pub fn parse(input: &str) -> Vec<Command> {
    all_consuming(separated_list0(line_ending, alt((cd_command, ls_command))))(input.trim())
        .unwrap()
        .1
}

type FileSystem = HashMap<Vec<String>, Vec<DirectoryListing>>;
type FolderSizes = HashMap<Vec<String>, u32>;

pub fn part1_and_2(input: &[Command]) -> (u32, u32) {
    let mut current_path = Vec::new();

    let mut file_system: FileSystem = HashMap::new();

    for cmd in input {
        match cmd {
            Command::LsCommand(content) => {
                file_system.insert(current_path.clone(), content.clone());
            }
            Command::CdCommand(destination) => match destination {
                CdCommandParam::Root => current_path.clear(),
                CdCommandParam::Up => {
                    current_path.pop();
                }
                CdCommandParam::Down(folder) => current_path.push(folder.clone()),
            },
        }
    }

    let mut folder_sizes: FolderSizes = HashMap::new();

    let total_size = calc_size(&[], &file_system, &mut folder_sizes);

    let part1 = folder_sizes.values().filter(|&&v| v < 100_000).sum();

    // part 2
    let free_space = 70_000_000 - total_size;
    let min_delete_folder_size = 30_000_000 - free_space;

    let part2 = *folder_sizes
        .values()
        .filter(|&&v| v >= min_delete_folder_size)
        .min()
        .unwrap();

    (part1, part2)
}

fn calc_size(
    current_path: &[String],
    file_system: &FileSystem,
    folder_sizes: &mut FolderSizes,
) -> u32 {
    let mut current_path = current_path.to_vec();
    let mut total_size = 0;

    for listing in file_system.get(&current_path).unwrap_or(&vec![]) {
        match listing {
            DirectoryListing::File(size, _) => total_size += size,
            DirectoryListing::Directory(name) => {
                current_path.push(name.clone());
                total_size += calc_size(&current_path, file_system, folder_sizes);
                current_path.pop();
            }
        }
    }

    folder_sizes.insert(current_path, total_size);

    total_size
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run07() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1_and_2(&parsed));
    }

    #[test]
    fn test_cd_command_parser() {
        assert_eq!(
            cd_command("$ cd /"),
            Ok(("", Command::CdCommand(CdCommandParam::Root)))
        );
        assert_eq!(
            cd_command("$ cd .."),
            Ok(("", Command::CdCommand(CdCommandParam::Up)))
        );
        assert_eq!(
            cd_command("$ cd foobar"),
            Ok((
                "",
                Command::CdCommand(CdCommandParam::Down("foobar".into()))
            ))
        );
        assert!(cd_command("$ cd 123").is_err());
    }

    #[test]
    fn test_dir_listing() {
        assert_eq!(
            directory_listing("dir foobar"),
            Ok(("", DirectoryListing::Directory("foobar".into())))
        );
        assert_eq!(
            directory_listing("1234 foo.bar"),
            Ok(("", DirectoryListing::File(1234, "foo.bar".into())))
        );
        assert!(directory_listing("12.34 foo$bar").is_err());
    }

    #[test]
    fn test_ls_command() {
        assert_eq!(
            ls_command("$ ls\n1234 foo.bar\ndir foobar"),
            Ok((
                "",
                Command::LsCommand(vec![
                    DirectoryListing::File(1234, "foo.bar".into()),
                    DirectoryListing::Directory("foobar".into()),
                ])
            ))
        );
    }
}
