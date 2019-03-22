var b = require('bonescript');
var net = require('net');
var LEDS = [
  'USR0',
  'USR1',
  'USR2',
  'USR3'
];
LEDS.forEach(function (LED) {
  b.pinMode(LED, b.OUTPUT);
  b.digitalWrite(LED, b.LOW);
});
var server = net.createServer(function(connection) {
  LEDS.forEach(function (LED) {
    b.digitalWrite(LED, b.HIGH);
  });
  connection.end();
});

server.listen(9999, function () {});
