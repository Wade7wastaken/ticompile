var Item = {Group: "group", Text: "text", Formula: "formula"};

let data = {
	type: Item.Group,
	name: "Choose an Option",
	contents: [
		{
			type: Item.Group,
			name: "Algebra",
			contents: [
				{
					type: Item.Text,
					name: "Perfect square trinomial",
					lines: ["(a+b)^^2=a^^2+2ab+b^^2", "a^^2+b^^2=(a+b)^^2-2ab"],
				},
				{
					type: Item.Text,
					name: "Difference of squares",
					lines: ["(a+b)(a-b)=a^^2+b^^2"],
				},
				{
					type: Item.Text,
					name: "Sum/difference of cubes",
					lines: ["a^^3+-b^^3=(a+-b)(a^^2-+ab+b^^2)"],
				},
				{
					type: Item.Text,
					name: "Binomial theorem",
					lines: [
						"kth term of (x+y)^n",
						"Start at k=0",
						"nCk*x^(n-k)*y^k",
					],
				},
				{
					type: Item.Text,
					name: "Fractional exponents",
					lines: ["a^(p/q)=qth root of a^p"],
				},
				{
					type: Item.Group,
					name: "Sequences",
					contents: [
						{
							type: Item.Group,
							name: "Arithmetic",
							contents: [
								{
									type: Item.Text,
									name: "Definition",
									lines: [
										"Sequence of numbers that",
										"starts at a1 and increases",
										"by d every iteration",
									],
								},
								{
									type: Item.Text,
									name: "General term",
									lines: ["1st term n=1", "Sn=S1+(n-1)d"],
								},
								{
									type: Item.Text,
									name: "Finite sum",
									lines: [
										"a1: 1st term",
										"an: last term",
										"n: # terms",
										"(n/2)(a1+an)",
									],
								},
							],
						},
						{
							type: Item.Group,
							name: "Geometric",
							contents: [
								{
									type: Item.Text,
									name: "General term",
									lines: ["Sn=S1r^(n-1)"],
								},
								{
									type: Item.Text,
									name: "Finite sum",
									lines: ["Sn=S1(1-r^n)/(1-r)"],
								},
								{
									type: Item.Text,
									name: "Infinite sum",
									lines: ["Sinf=S1/(1-r)"],
								},
							],
						},
					],
				},
			],
		},
		{
			type: Item.Group,
			name: "Geometry",
			contents: [
				{
					type: Item.Group,
					name: "Circle",
					contents: [
						{
							type: Item.Text,
							name: "Definition",
							lines: [
								"Points same dist from center",
								"(x-h)^^2+(y-k^^2)=r^^2",
								"k,h=center",
								"r=radius",
							],
						},
						{
							type: Item.Formula,
							name: "Area",
							vars: [
								{ letter: "A", name: "Area", inputted: false },
								{ letter: "r", name: "Radius", inputted: true },
							],
							lines: ["A=pir^^2"],
							expression: ["piR^^2"],
						},
						{
							type: Item.Formula,
							name: "Perimeter",
							vars: [
								{
									letter: "P",
									name: "Perimeter",
									inputted: false,
								},
								{ letter: "r", name: "Radius", inputted: true },
							],
							lines: ["P=2pir"],
							expression: ["2piR"],
						},
					],
				},
				{
					type: Item.Group,
					name: "Triangle",
					contents: [
						{
							type: Item.Formula,
							name: "Area",
							vars: [
								{ letter: "A", name: "Area", inputted: false },
								{ letter: "B", name: "Base", inputted: true },
								{ letter: "h", name: "Height", inputted: true },
							],
							lines: ["A=1/2Bh"],
							expression: ["0.5BH"],
						},
						{
							type: Item.Formula,
							name: "Area equilateral",
							vars: [
								{ letter: "A", name: "Area", inputted: false },
								{
									letter: "s",
									name: "Side length",
									inputted: true,
								},
							],
							lines: ["A=s^^2sqrt(3)/4"],
							expression: ["S^^2*sqrt(3)/4"],
						},
						{
							type: Item.Formula,
							name: "Heron's form\\ula",
							vars: [
								{ letter: "A", name: "Area", inputted: false },
								{
									letter: "S",
									name: "Semi-perimeter",
									inputted: false,
								},
								{
									letter: "x",
									name: "Side length",
									inputted: true,
								},
								{
									letter: "y",
									name: "Side length",
									inputted: true,
								},
								{
									letter: "z",
									name: "Side length",
									inputted: true,
								},
							],
							lines: ["A=sqrt(S(S-x)(S-y)(S-z))", "S=(x+y+z)/2"],
							expression: [
								"(X+Y+Z)/2->S",
								"sqrt(S(S-X)(S-Y)(S-Z))",
							],
						},
						{
							type: Item.Formula,
							name: "Altitude scalene",
							vars: [
								{
									letter: "H",
									name: "Altitude",
									inputted: false,
								},
								{ letter: "A", name: "Area", inputted: true },
								{ letter: "b", name: "Base", inputted: true },
							],
							lines: ["Use heron's for area", "H=2A/b"],
							expression: ["2A/B"],
						},
						{
							type: Item.Formula,
							name: "Altitude isosceles",
							vars: [
								{
									letter: "H",
									name: "Altitude",
									inputted: false,
								},
								{ letter: "l", name: "Leg", inputted: true },
								{ letter: "b", name: "Base", inputted: true },
							],
							lines: ["H=sqrt(a^^2-(b^^2/4))"],
							expression: ["sqrt(A^^2-(B^^2/4))"],
						},
						{
							type: Item.Formula,
							name: "Altitude equilateral",
							vars: [
								{
									letter: "H",
									name: "Altitude",
									inputted: false,
								},
								{
									letter: "a",
									name: "Side length",
									inputted: true,
								},
							],
							lines: ["H=asqrt(3)/2"],
							expression: ["Asqrt(3)/2"],
						},
					],
				},
			],
		},
		{
			type: Item.Group,
			name: "Algebra 2",
			contents: [
				{
					type: Item.Text,
					name: "Log conversion",
					lines: ["log base b(a)=x", "b^x=a", "Only if x>0,b>0,b!=1"],
				},
				{
					type: Item.Text,
					name: "Sum/prod of roots",
					lines: [
						"Given polynomial",
						"an^n+a(n-1)^(n-1)...+a0",
						"Sum:  -a(n-1)/an",
						"Prod: a0/an if n even",
						"      -a0/an if n odd",
					],
				},
			],
		},
		{
			type: Item.Group,
			name: "Trigonometry",
			contents: [
				{
					type: Item.Text,
					name: "Trig Definitions",
					lines: [
						"Circle: x^^2+y^^2=r^^2",
						"sin=y/r  csc=r/y",
						"cos=x/r  sec=r/x",
						"tan=y/x  cot=x/y",
					],
				},
				{
					type: Item.Text,
					name: "Trig ratios",
					lines: [
						"sin=opp/hyp",
						"cos=adj/hyp",
						"tan=opp/adj",
						"csc=1/sin",
						"sec=1/cos",
						"cot=1/tan",
						"tan=sin/cos cot=cos/sin",
					],
				},
				{
					type: Item.Text,
					name: "Pythagorean identities",
					lines: [
						"sin^^2+cos^^2=1",
						"1=cot^^2=csc^^2",
						"tan^^2+1=sec^^2",
					],
				},
				{
					type: Item.Text,
					name: "Cofunctions",
					lines: [
						"sin(90^^o-theta)=cos(theta)",
						"sec(90^^o-theta)=csc(theta)",
						"tan(90^^o-theta)=cot(theta)",
					],
				},
				{
					type: Item.Text,
					name: "Even/Odd",
					lines: [
						"Even:       Odd:",
						"cos-theta=costheta  sin-theta=-sintheta",
						"sec-theta=sectheta  csc-theta=-csctheta",
						"            tan-theta=-tantheta",
						"            cot-theta=-cottheta",
					],
				},
				{
					type: Item.Text,
					name: "Law of sines",
					lines: ["sinA/a=sinB/b=sinC/c"],
				},
				{
					type: Item.Text,
					name: "Law of cosines",
					lines: [
						"a^^2=b^^2+c^^2-2bc*cosA",
						"b^^2=a^^2+c^^2-2ac*cosB",
						"c^^2=a^^2+b^^2-2ab*cosC",
						"",
						"cosA=(b^^2+c^^2-a^^2)/2bc",
						"cosB=(a^^2+c^^2-b^^2)/2ac",
						"cosC=(a^^2+b^^2-c^^2)/2ab",
					],
				},
				{
					type: Item.Text,
					name: "Sum form\\ulas",
					lines: [
						"sin(A+-B)=",
						"    sinAcosB+-cosAsinB",
						"cos(A+-B)=",
						"    cosAcosB-+sinAsinB",
						"tan(a+-B)=(tanA+-tanB)/",
						"    (1-+tanAtanB)",
					],
				},
				{
					type: Item.Text,
					name: "Double angle form\\ulas",
					lines: [
						"sin(2A)=2sinAcosA",
						"cos(2A)=cos^^2A-sin^^2A",
						"       =2cos^^2A-1",
						"       =1-2sin^^2A",
						"tan(2A)=(2tanA)/(1-tan^^2A)",
					],
				},
			],
		},
	],
};

console.log(JSON.stringify(data))