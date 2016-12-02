<?php

function calculateCode($rows, $fieldMapping, $initialPosition)
{
  $moveMapping = [
    'U' => ['x' => 0, 'y' => 1],
    'D' => ['x' => 0, 'y' => -1],
    'L' => ['x' => -1, 'y' => 0],
    'R' => ['x' => 1, 'y' => 0]
  ];
  $currentPosition = $initialPosition;
  $code = '';

  foreach ($rows as $row) {
    foreach ($row as $instruction) {
      $movevector = $moveMapping[$instruction];
      $nextPosition = [
        'x' => $currentPosition['x'] + $movevector['x'],
        'y' => $currentPosition['y'] + $movevector['y'],
      ];
      $field = sprintf('%s,%s', $nextPosition['x'], $nextPosition['y']);

      if (isset($fieldMapping[$field])) {
        $currentPosition = $nextPosition;
      } else {
        // ignore instruction
      }
    }
    $field = sprintf('%s,%s', $currentPosition['x'], $currentPosition['y']);
    $code = sprintf('%s%s', $code, $fieldMapping[$field]);
  }
  return $code;
}

$inputString = trim(file_get_contents('input.txt'));
//$inputString = "ULL\nRRDDD\nLURDL\nUUUUD";
$rows = array_map('str_split', explode("\n", $inputString));


$fieldMapping1 = [
  '0,0' => 1,
  '1,0' => 2,
  '2,0' => 3,
  '0,-1' => 4,
  '1,-1' => 5,
  '2,-1' => 6,
  '0,-2' => 7,
  '1,-2' => 8,
  '2,-2' => 9,
];

$fieldMapping2 = [
  '2,0' => '1',
  '1,-1' => '2',
  '2,-1' => '3',
  '3,-1' => '4',
  '0,-2' => '5',
  '1,-2' => '6',
  '2,-2' => '7',
  '3,-2' => '8',
  '4,-2' => '9',
  '1,-3' => 'A',
  '2,-3' => 'B',
  '3,-3' => 'C',
  '2,-4' => 'D',
];


var_dump('part1:', calculateCode($rows, $fieldMapping1, ['x' => 1, 'y' => -1]));
var_dump('part2:', calculateCode($rows, $fieldMapping2, ['x' => 0, 'y' => -2]));
