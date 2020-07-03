`default_nettype none
module basic_gates
(
	input in1,
	input in2,
	input in3,
	input in4,
	input in5,
	input in6,
	output out1,
	output out2);

	wire w1 = !(in1 & in2);
	wire w2 = w1 | in3;
	wire w3 = in4 | in5;
	wire w4 = w2 | !w3;
	assign out1 = w3 ^ w1;
	assign out2 = in6 & w4;
endmodule
