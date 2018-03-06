min=47
max=55
for i in `seq $min $max`
do
    cargo run --release -- --posgen 100000 $i $i'_random_unsolved_2.json'
    cargo run --release -- --fullsolve $i'_random_unsolved_2.json' $i'_random_solved_2.json' &
done
