# frozen_string_literal: true

require 'fileutils'

if ARGV.length != 1
  puts 'Usage: ruby scaffold_day.rb <DAY_NUMBER>'
  exit 1
end

day_nr = ARGV[0]

puts 'Creating input directory...'
FileUtils.mkdir_p("inputs/day#{day_nr}")

puts 'Creating scaffold Rust file...'
File.open("src/bin/day#{day_nr}.rs", 'w') do |f|
  f << <<~HEREDOC
    fn main() {
        println!("Day #{day_nr}, part 1: {}", part1());
        println!("Day #{day_nr}, part 2: {}", part2());
    }

    fn part1() -> i32 {
      unimplemented!();
    }
    fn part2() -> i32 {
      unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::{fs::File, io::BufReader};

        #[test]
        fn test_solution() {
        }
    }
  HEREDOC
end

puts 'Adding entry in Cargo.toml...'
File.open('Cargo.toml', 'a') do |f|
  f << <<~HEREDOC

    [[bin]]
    name = "day#{day_nr}"
    path = "src/bin/day#{day_nr}.rs"
  HEREDOC
end

puts 'Done! Happy Advent!'
