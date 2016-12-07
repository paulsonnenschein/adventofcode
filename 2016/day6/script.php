<?php

$inputString = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

$inputString = file_get_contents('input.txt');

$cols = 8;

$input = array_map('str_split', explode("\n", trim($inputString)));

$result = [];
$resultStr = '';
$resultStr2 = '';
for ($i = 0; $i < $cols; $i++) {
  $result[$i] = [];
  $column = array_column($input, $i);
  foreach ($column as $char) {
    if (!isset($result[$i][$char])) {
      $result[$i][$char] = 0;
    }
    $result[$i][$char]++;
  }

  arsort($result[$i]);
  $resultStr .= array_keys($result[$i])[0];

  asort($result[$i]);
  $resultStr2 .= array_keys($result[$i])[0];
}

var_dump('part1:', $resultStr);
var_dump('part2:', $resultStr2);
