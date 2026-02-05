const command = process.argv[2];

switch (command) {
  case 'hello':
    const message = process.argv[3];
    console.log(`Hello ${message}!`);
    break;
  default:
    console.error(`unknown command ${command}`);
    process.exit(1);
}