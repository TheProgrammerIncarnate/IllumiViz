`default_nettype none
module wide_gates
(
	input in1,
	input in2,
	input in3,
	input in4,
	input in5,
	input in6,
	input in7,
	input in8,
	output out1,
	output out2,
	output all_high);

	wire w1 = in1 | in2 | in3;
	wire w2 = in4 & w1;
	assign out1 = w2 & in7;
	assign out2 = (in5 & in6) | in7;
	assign all_high = in1 & in2 & in3 & in4 & in5 & in6 & in7 & in8;
endmodule
