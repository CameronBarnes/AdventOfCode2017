set dotenv-load := true

alias w := work
work day:
		nvim day-{{day}}

alias c := create
create day:
		cargo generate --path ./daily-template --name day-{{day}}
		bash -c "curl 'https://adventofcode.com/$YEAR/day/{{day}}/input' -H 'cookie: session=$AOC_SESSION' -H 'user-agent: one time download script. Author: cameron_barnes@outlook.com' --compressed > day-{{day}}/input.txt"

alias ba := bench-all
bench-all:
		cargo bench -q > benchmarks.txt
alias b := bench
bench day part:
		cargo bench --bench day-{{day}}-bench part{{part}} >> day-{{day}}.bench.txt

flamegraph day part:
		cargo flamegraph --profile flamegraph --root --package day-{{day}} --bin part{{part}} -o flamegraphs/day-{{day}}--part{{part}}.svg

test-all:
		cargo nextest r
alias t := test
test day part:
		cargo nextest r part{{part}} -p day-{{day}}
test-day day:
		cargo nextest r -p day-{{day}}

alias r := run
run day part:
		cargo run -p day-{{day}} --bin part{{part}}
