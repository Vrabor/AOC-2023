const fs = require('node:fs');

let input = fs.readFileSync('input.txt', 'utf8').split('\n');
const numbers = input.map((line) => Array.from(line.matchAll(/(\d+)/g)));
const symbols = input.map((line) => Array.from(line.matchAll(/([^\d|\.])/g)));

function adjacent(number, symbol){
  return symbol.index >= number.index - 1 
    && symbol.index <= number.index + number[0].length;
}

function has_symbol(num, i){
  if (i > 0 && symbols[i - 1].findIndex((x) => {return adjacent(num, x)}) > -1){
    return true;
  }
  if (i < symbols.length && symbols[i + 1].findIndex((x) => {return adjacent(num, x)}) > -1 ){
    return true;
  }
  return symbols[i].findIndex((x) => {return adjacent(num, x)}) > -1;
}

let part1 = 0;
numbers.forEach((line, i) => {
  line.forEach((num) => {
      if (has_symbol(num, i)) {
        part1 += Number(num[0]);
      }
    }
  )}
)
console.log('Part1: ' + part1);

const gears = input.map((line) => Array.from(line.matchAll(/\*/g)));

function adjacent_nums(gear, i){
  let possible = numbers[i]
  if (i > 0) possible = possible.concat(numbers[i - 1]);
  if (i < numbers.length) possible = possible.concat(numbers[i + 1]);
  return possible.filter((x) => adjacent(x, gear));
}

part2 = 0;
gears.forEach((line, li) => {
  line.forEach((gear) => {
    let n = adjacent_nums(gear, li);
    if (n.length == 2) part2 += Number(n[0][0]) * Number(n[1][0]);
  })
})

console.log('Part2: ' + part2);
