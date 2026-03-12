const sleep = async (time : number, func : Function = () => {}, para : Array<any> = []) : Promise<void> => {
	const data = Date.now();
	await func(...para);
	return new Promise(resolve => setTimeout(resolve, Math.max(0, time - (Date.now() - data))));
};

export default sleep;