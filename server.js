var b = require('bonescript');
var net = require('net');
var LEDS = [
  'USR0',
  'USR1',
  'USR2',
  'USR3'
];
var state = b.LOW;
LEDS.forEach(function (LED) {
  b.pinMode(LED, b.OUTPUT);
  b.digitalWrite(LED, state);
});
var server = net.createServer(function(connection) {
  console.log('Received connection');
  state = Number(!Boolean(state));
  LEDS.forEach(function (LED) {
    b.digitalWrite(LED, state);
  });
  connection.end();
});
server.listen(9999, function () {
  console.log('Ready for commands');
});
