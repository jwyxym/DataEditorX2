class DB {
	name : string;
	path : string;
	cards ?: Array<[number, string]>;
	constructor (path : string) {
		const arr = path.split(/[\\/]/);
		this.path = path;
		this.name = arr[arr.length - 1];
	}
};

export default DB;