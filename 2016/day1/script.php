<?php


function nextDirection($direction, $curentDirection) {
  if ($direction === 'R') {
    switch ($curentDirection) {
      case 'up': return 'right';
      case 'right': return 'down';
      case 'down': return 'left';
      case 'left': return 'up';
      default: throw new Exception("Invalid");
    }
  } else {
    switch ($curentDirection) {
      case 'up': return 'left';
      case 'left': return 'down';
      case 'down': return 'right';
      case 'right': return 'up';
      default: throw new Exception("Invalid");
    }
  }
}

function createVector($direction, $distance) {
  switch ($direction) {
    case 'up': return ['x' => $distance, 'y' => 0];
    case 'right': return ['x' => 0, 'y' => $distance];
    case 'down': return ['x' => -$distance, 'y' => 0];
    case 'left': return ['x' => 0, 'y' => -$distance];
    default: throw new Exception("Invalid");
  }
}

$fileContent = trim(file_get_contents('input.txt'));
//$fileContent = 'R8, R4, R4, R8';
$instructions = explode(', ', $fileContent);



// up, down, left, right

$currentDirection = 'up';
$vectors = [];
foreach ($instructions as $instructionString) {
  $direction = $instructionString[0];
  $distance = substr($instructionString, 1);

  $currentDirection = nextDirection($direction, $currentDirection);
  $vector = createVector($currentDirection, $distance);

  $vectors[] = $vector;
}

$lastCoord = array_reduce($vectors, function ($carry, $item) {
  return ['x' => $carry['x'] + $item['x'], 'y' => $carry['y'] + $item['y']];
}, ['x' => 0, 'y' => 0]);

var_dump('part1:', abs($lastCoord['x']) + abs($lastCoord['y']));
