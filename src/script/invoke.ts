import { invoke as tauri_invoke } from '@tauri-apps/api/core';
import toast from './toast';
import * as bincode from 'bincode-ts';

class Invoke {
	init = async () : Promise<boolean> => {
		try {
			await tauri_invoke('init');
		} catch (e) {
			toast.error(e)
			return false;
		}
		return true;
	};
	read_db = async (path : string) : Promise<boolean> => {
		try {
			await tauri_invoke('read_db', { path : path });
		} catch (e) {
			toast.error(e)
			return false;
		}
		return true;
	};
	write_db = async (path : string, code : number, cards : [Array<number>, Array<string>]) : Promise<boolean> => {
		try {
			console.log(path, cards, code)
			await tauri_invoke('write_db', { path : path, code : code, cdb : cards });
		} catch (e) {
			toast.error(e)
			return false;
		}
		return true;
	};
	del_db = async (path : string, code : number) : Promise<boolean> => {
		try {
			await tauri_invoke('del_db', { path : path, code : code });
		} catch (e) {
			toast.error(e)
			return false;
		}
		return true;
	};
	create_db = async (path : string) : Promise<boolean> => {
		try {
			await tauri_invoke('create_db', { path : path });
		} catch (e) {
			toast.error(e)
			return false;
		}
		return true;
	};
	close_db = async (path : string) : Promise<boolean> => {
		try {
			await tauri_invoke('close_db', { path : path });
		} catch (e) {
			toast.error(e)
			return false;
		}
		return true;
	};
	get_db = async (path : string, code : number) : Promise<{path : string; lua : [string, string]; card : [Array<number>, Array<string>]} | undefined> => {
		try {
			const result = await tauri_invoke<ArrayBuffer>('get_db', { path : path, code : code });
			return bincode.decode(bincode.Struct({
				path : bincode.String,
				lua : bincode.Tuple(bincode.String, bincode.String),
				card : bincode.Tuple(
					bincode.Collection(bincode.u32),
					bincode.Collection(bincode.String)
				)
			}), result).value as any;
		} catch (e) {
			toast.error(e)
			return undefined;
		}
	};
	get_list = async (path : string) : Promise<Array<[number, string]>> => {
		try {
			const result = await tauri_invoke<ArrayBuffer>('get_list', { path : path });
			return bincode.decode(bincode.Collection(
				bincode.Tuple(bincode.u32, bincode.String)
			), result).value as Array<[number, string]>;
		} catch (e) {
			toast.error(e)
			return [];
		}
	};
	get_dbs = async () : Promise<Array<string>> => {
		try {
			return await tauri_invoke<Array<string>>('get_dbs');
		} catch (e) {
			toast.error(e)
			return [];
		}
	};
	get_config = async () : Promise<{
			ot : Array<[number, string]>,
			attribute : Array<[number, string]>,
			category : Array<[number, string]>,
			race : Array<[number, string]>,
			types : Array<[number, string]>
		}> => {
		try {
			const result = await tauri_invoke<ArrayBuffer>('get_config');
			return bincode.decode(bincode.Struct({
					ot : bincode.Collection(bincode.Tuple(bincode.u32, bincode.String)),
					attribute : bincode.Collection(bincode.Tuple(bincode.u32, bincode.String)),
					category : bincode.Collection(bincode.Tuple(bincode.u32, bincode.String)),
					race : bincode.Collection(bincode.Tuple(bincode.u32, bincode.String)),
					types : bincode.Collection(bincode.Tuple(bincode.u32, bincode.String))
				}), result).value as any;
		} catch (e) {
			toast.error(e)
			return {
				ot : [],
				attribute : [],
				category : [],
				race : [],
				types : []
			};
		}
	};
	save_lua = async (path : string, lua : string) : Promise<boolean> => {
		try {
			await tauri_invoke<ArrayBuffer>('save_lua', { path : path, lua : lua });
			return true;
		} catch (e) {
			toast.error(e)
			return false;
		}
	};
};
const invoke = new Invoke();
export default invoke;