function main() {
  let pi = 0;
  for (let i = 0; i < 100_000_000; i++) {
    let numerator = i % 2 === 0 ? 4.0 : -4.0;
    let dominator = 2.0 * i + 1.0;
    pi += numerator / dominator;
  }
  console.log(pi);
}

main();
