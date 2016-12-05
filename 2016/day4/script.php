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

  return [$matches[1], sortStr($rowStr), $matches[2], $matches[3]];
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
  return isValid($row[1], $row[3]);
});

$result = array_sum(array_column($filtered, 2));

var_dump('part1:', $result);

function str_rot($s, $n = 13) {
    static $letters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    $n = (int)$n % 26;
    if (!$n) return $s;
    if ($n == 13) return str_rot13($s);
    for ($i = 0, $l = strlen($s); $i < $l; $i++) {
        $c = $s[$i];
        if ($c >= 'a' && $c <= 'z') {
            $s[$i] = $letters[(ord($c) - 71 + $n) % 26];
        } else if ($c >= 'A' && $c <= 'Z') {
            $s[$i] = $letters[(ord($c) - 39 + $n) % 26 + 26];
        }
    }
    return $s;
}

function decrypt($str, $id) {
  return str_rot(str_replace('-', ' ', $str), $id % 26);
}


foreach ($filtered as $row) {
  $decrypted = decrypt($row[0], (int)$row[2]);
  if ($decrypted === 'northpole object storage') {
    var_dump('part2:', $row[2]);
    break;
  }
}
