
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
});

var on_led_index = 0;

setInterval(function () {
  b.digitalWrite(LEDS[on_led_index], b.LOW);
  on_led_index = (on_led_index + 1) % 4;
  b.digitalWrite(LEDS[on_led_index], b.HIGH);
}, 1000);


setInterval(function () {}, 5000);

