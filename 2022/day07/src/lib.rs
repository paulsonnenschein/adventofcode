use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, not_line_ending, u32 as parse_u32},
    combinator::{all_consuming, map, value},
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

    let parse_root = value(Command::CdCommand(CdCommandParam::Root), tag("/"));
    let parse_up = value(Command::CdCommand(CdCommandParam::Up), tag(".."));
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

type FolderSizes = HashMap<Vec<String>, u32>;

pub fn part1_and_2(input: &[Command]) -> (u32, u32) {
    let mut current_path = Vec::new();

    let mut folder_sizes: FolderSizes = HashMap::new();

    for cmd in input {
        match cmd {
            Command::LsCommand(content) => {
                let key = current_path.clone();
                if let std::collections::hash_map::Entry::Vacant(e) = folder_sizes.entry(key) {
                    let local_size = content
                        .iter()
                        .filter_map(|item| match item {
                            DirectoryListing::File(size, _) => Some(size),
                            DirectoryListing::Directory(_) => None,
                        })
                        .sum();
                    e.insert(local_size);

                    // update parent
                    let mut parent_key = current_path.clone();
                    while !parent_key.is_empty() {
                        parent_key.pop();
                        *folder_sizes.get_mut(&parent_key).unwrap() += local_size;
                    }
                }
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

    let total_size = folder_sizes.get(&vec![]).unwrap();

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
