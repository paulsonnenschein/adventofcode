<?php

$inputString = trim(file_get_contents('input.txt'));

$rows = explode("\n", $inputString);

$rows = array_map(function ($row) {
  preg_match('/ *?(\d+) *?(\d+) *?(\d+)/', $row, $matches);
  return [(int)$matches[1],(int)$matches[2],(int)$matches[3]];
}, $rows);

function isValid($a, $b, $c) {
  return ($a + $b > $c) && ($a + $c > $b) && ($b + $c > $a);
}

$filtered = array_filter($rows, function ($row) {
  [$a, $b, $c] = $row;
  return isValid($a, $b, $c);
});

var_dump('part1:', count($filtered));

$chunked = array_chunk($rows, 3);

$chunkedTurned = array_map(function ($row) {
  return [
    [$row[0][0], $row[1][0], $row[2][0]],
    [$row[0][1], $row[1][1], $row[2][1]],
    [$row[0][2], $row[1][2], $row[2][2]],
  ];
}, $chunked);

$rowsTurned = array_merge(...$chunkedTurned);

$filtered2 = array_filter($rowsTurned, function ($row) {
  [$a, $b, $c] = $row;
  return isValid($a, $b, $c);
});

var_dump('part2:', count($filtered2));
