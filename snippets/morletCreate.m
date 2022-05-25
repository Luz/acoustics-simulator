% Morlet wave packet, amplitude calculation in octave

% Define window
maxtime = 0.00016; % 160 ns
delay = maxtime/2; % 80 ns
resolution = 0.000001; % 1 ns
time = 0:resolution:maxtime;

% Define wave packet
wave_frequency = 40000;
wave_cycles = 7;

% time: time in seconds
% delay: delay in seconds
% cycles: number of cycles, e.g. chose 7 for rawly ~7 peaks
% frequency: frequency of the wavelet
function y = morletpacket(time, delay, cycles, frequency)
	sigma = cycles/(2*pi*frequency);
	y = exp(-0.5*((time-delay)/sigma).^2) .* cos(2*pi*frequency*(time-delay));
endfunction

ampl = morletpacket(time, delay, wave_cycles, wave_frequency);

h = figure();
plot(time, ampl);
xlim([0, maxtime]);

saveas(h, "morletPacket.png");

