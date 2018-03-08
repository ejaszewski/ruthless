min=48
max=49
for i in `seq $min $max`
do
    for j in `seq 0 7`
    do
        cargo run --release --bin ruthless -- --posgen 25000 $i $i'_random_unsolved_'$j'.json'
        cargo run --release --bin ruthless -- --fullsolve $i'_random_unsolved_'$j'.json' $i'_random_solved_'$j'.json' &
    done
done
