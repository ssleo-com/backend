// Still needs implementation of differentthan, contains, startsWith, endsWith, lessthan, greaterthan, lessthanorequal, greaterthanorequal, isEmpty, isNotEmpty
// Also the extra filter such as: person has 3 phone numbers etc

// Name = Josh OR Age = 50
const filter = [
	{
		name: "Josh",
	},
	{ age: 50 },
];

// Name = Josh AND Age = 50
const filter2 = [{ name: "Josh", age: 50 }];

// (Name = Josh AND Gender = male) OR Age = 50
const filter3 = [{ name: "Josh", gender: "male" }, { age: 50 }];

// (Name = Josh AND (Gender = male OR nationality = USA)) OR Age = 50
const filter4 = [
	{ name: "Josh", filters: [{ gender: "male" }, { nationality: "USA" }] },
	{ age: 50 },
];
