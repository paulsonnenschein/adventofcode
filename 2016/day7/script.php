<?php

$inputString = 'abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn';

$inputString = file_get_contents('input.txt');

$input = explode("\n", trim($inputString));

$i = 0;
foreach ($input as $row) {
  if (preg_match('/\[[a-z]*([a-z])(?!\1)([a-z])\2\1[a-z]*\]/', $row)) {
    continue;
  }

  if (preg_match('/([a-z])(?!\1)([a-z])\2\1/', $row)) {
    $i++;
  }
}

var_dump('part1:', $i);

$part2regex = '/([a-z])((?!\1)[a-z])\1(?:[a-z]|\[[a-z]*\])*\[[a-z]*\2\1\2|([a-z])((?!\3)[a-z])\3[a-z]*\](?:[a-z]|\[[a-z]*\])*\4\3\4/';

$i = 0;
foreach ($input as $row) {
  if (preg_match($part2regex, $row, $matches)) {
    $i++;
  }
}

var_dump('part2:', $i);
