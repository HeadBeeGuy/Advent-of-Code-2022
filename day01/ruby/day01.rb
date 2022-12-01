# I thought I was doing this problem wrong, so I whipped this up in Ruby.
# But then I got the same answer, so I input it again, and it worked!
# I think I just entered it incorrectly when I did it in Rust.

input = File.open("../input.txt").readlines.map(&:strip)

meals = []
input.slice_when { |a| a.empty? }.each { |arr| meals << arr.map(&:to_i) }

# slice_when still leaves the "" element in place, and to_i converts it to 0, which
# means it's one of the elements that's summed up. It won't hurt things, and maybe
# it's faster than trying to remove the 0 element before doing the sum.

sums = meals.map(&:sum).sort

puts sums[-1]
puts sums[-3..-1].sum