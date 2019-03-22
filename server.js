var b = require('bonescript');
var net = require('net');

var LEDS = [
  'USR0',
  'USR1',
  'USR2',
  'USR3'
];
var open_sockets = [];
var state = b.HIGH;
function setState(new_state) {
  if (new_state !== state) {
    state = new_state;
    LEDS.forEach(function (LED) {
      b.pinMode(LED, b.OUTPUT);
      b.digitalWrite(LED, state);
    });
    writeStateAllSockets();
    return true;
  } else {
    return false;
  }
}
setState(b.LOW);
function writeState(socket) {
  socket.write(state + '\n');
}
function writeStateAllSockets() {
  open_sockets.forEach(writeState);
}
setInterval(writeStateAllSockets, 10000);
var server = net.createServer(function (socket) {
  writeState(socket);
  open_sockets.push(socket);
  socket.on('end', function () {
    open_sockets = open_sockets.filter(function (remove_socket) {
      return remove_socket !== socket;
    });
  });
  socket.on('error', function (error) {
    console.log('error', error);
  });
  socket.on('data', function (data) {
    var command_character = data.slice(0, 1);
    var ascii_signal = parseInt(command_character.toString('ascii'), 10);
    var state_changed = false;
    if (ascii_signal === 1) {
      state_changed = setState(b.HIGH);
    } else if (ascii_signal === 0) {
      state_changed = setState(b.LOW);
    } else {
      socket.write('!\n');
      return;
    }
    if (!state_changed) {
      writeState(socket);
    }
  });
});
server.listen(9999, function () {
  console.log('Ready for commands');
});
