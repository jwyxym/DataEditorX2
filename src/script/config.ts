import invoke from "./invoke";

class Config {
    ot : Array<[number, string]> = [];
    attribute : Array<[number, string]> = [];
    category : Array<[number, string]> = [];
    race : Array<[number, string]> = [];
    types : Array<[number, string]> = [];
    
    init = async () => {
        const [_, i] = await Promise.all([
			invoke.init(),
			invoke.get_config(),
		]);
        this.ot = i.ot;
        this.attribute = i.attribute;
        this.category = i.category;
        this.race = i.race;
        this.types = i.types;
    };
}

const config = new Config();
export default config;