<?php

$stringInput = trim(file_get_contents('input.txt'));
//$stringInput = "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1";


$lines = explode("\n", $stringInput);

function debug($display) {
  foreach ($display as $row) {
    foreach ($row as $field) echo $field ? '#' : '.';
    echo "\n";
  }
}

$display = array_fill(0, 6, array_fill(0, 50, false));
//$display = array_fill(0, 3, array_fill(0, 7, false));

foreach ($lines as $line) {
  $instructions = explode(' ', $line);

  if ($instructions[0] === 'rect') {
    list($width, $height) = explode('x', $instructions[1]);

    for ($x = 0; $x < $width; $x++) {
      for ($y = 0; $y < $height; $y++) {
        $display[$y][$x] = true;
      }
    }

  } elseif ($instructions[1] === 'row') {
    $y = (int)explode('=', $instructions[2])[1];
    $amount = (int)$instructions[4];

    $row = $display[$y];

    $newRow = array_merge(array_slice($row, -$amount), array_slice($row, 0, -$amount));

    /*foreach ($row as $field) echo $field ? '#' : '.';
    echo "\n";
    foreach ($newRow as $field) echo $field ? '#' : '.';
    echo "\n";*/

    $display[$y] = $newRow;

  } elseif ($instructions[1] === 'column') {
    $x = (int)explode('=', $instructions[2])[1];
    $amount = (int)$instructions[4];

    $column = array_column($display, $x);

    $newColumn = array_merge(array_slice($column, -$amount), array_slice($column, 0, -$amount));

    /*foreach ($column as $field) echo $field ? '#' : '.';
    echo "\n";
    foreach ($newColumn as $field) echo $field ? '#' : '.';
    echo "\n";*/

    foreach ($display as $index => $row) {
      $display[$index][$x] = $newColumn[$index];
    }
  }
}

debug($display);

$count = 0;
foreach ($display as $row) {
  foreach ($row as $field) $count += $field ? 1 : 0;
}

var_dump('part1:', $count);
