<?php

$inputString = trim(file_get_contents('input.txt'));

//$inputString = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";

function sortStr($str) {
  $stringParts = str_split($str);
  sort($stringParts);
  return implode('', $stringParts);
}

$rows = array_map(function ($row) {
  preg_match('/([a-z]+(?:-[a-z]+)*?)-(\d+)\[([a-z]+)\]/', $row, $matches);
  $rowStr = str_replace('-', '', $matches[1]);

  return [sortStr($rowStr), $matches[2], $matches[3]];
}, explode("\n", $inputString));

function isValid($str, $checksum) {
  $counter = [];

  foreach (str_split($str) as $char) {
    if (!isset($counter[$char])) {
      $counter[$char] = 0;
    }
    $counter[$char]++;
  }

  $counter2 = [];
  foreach ($counter as $char => $count) {
    $counter2[$count][] = $char;
  }

  ksort($counter2);

  $counter2 = array_reverse($counter2, true);

  $calculated = '';
  foreach ($counter2 as $count => $chars) {
    sort($chars);
    foreach ($chars as $char) {
      if (strlen($calculated) < 5) {
        $calculated .= $char;
      } else {
        break;
      }
    }
  }

  return $calculated === $checksum;
}

$filtered = array_filter($rows, function ($row) {
  return isValid($row[0], $row[2]);
});

$result = array_sum(array_column($filtered, 1));

var_dump('part1:', $result);
