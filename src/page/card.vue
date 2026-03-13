<template>
	<div class = 'card'>
		<div class = 'no-scrollbar'>
			<div>
				<var-cell>
					<var-input
						placeholder = '名称'
						variant = 'outlined'
						v-model = 'card.name'
					/>
				</var-cell>
				<br/>
				<div>
					<img :src = 'card.pic'/>
					<div v-if = 'card.type.includes(0x4000000)'>
						<img
							v-for = 'i in arr.pic[0]'
							:src = 'i.url[card.link & i.link ? 1 : 0]'
							@click = 'card.update.link(i.link)'
							class = 'cursor'
						/>
						<div></div>
						<img
							v-for = 'i in arr.pic[1]'
							:src = 'i.url[card.link & i.link ? 1 : 0]'
							@click = 'card.update.link(i.link)'
							class = 'cursor'
						/>
					</div>
				</div>
				<var-cell>
					<var-input
						v-for = '(_, v) in new Array(4)'
						:placeholder = '`字段${v + 1}`'
						v-model = 'card.setcode[v]'
					/>
				</var-cell>
				<var-cell>
					<var-select placeholder = '属性' v-model = 'card.attribute'>
						<var-option
							v-for = 'i in config.attribute'
							:label = 'i[1]'
							:value = 'i[0]'
						/>
					</var-select>
					<var-select placeholder = '种族' v-model = 'card.race'>
						<var-option
							v-for = 'i in config.race'
							:label = 'i[1]'
							:value = 'i[0]'
						/>
					</var-select>
				</var-cell>
				<var-cell>
					<var-input
						placeholder = '攻击力'
						v-model = 'card.atk'
						type = 'number'
					/>
					<var-input
						placeholder = '守备力'
						v-model = 'card.def'
						type = 'number'
					/>
				</var-cell>
				<var-cell>
					<var-input
						placeholder = '等级'
						v-model = 'card.level'
						type = 'number'
					/>
					<var-input
						v-if = 'card.type.includes(0x1000000)'
						placeholder = '刻度'
						v-model = 'card.scale'
						type = 'number'
					/>
				</var-cell>
				<var-cell>
					<var-input
						placeholder = '卡号'
						v-model = 'card.id'
						type = 'number'
					/>
					<var-input
						placeholder = '同名卡'
						v-model = 'card.alias'
						type = 'number'
					/>
				</var-cell>
			</div>
			<div>
				<var-cell>
					<var-select placeholder = 'OT' multiple v-model = 'card.ot'>
						<var-option
							v-for = 'i in config.ot'
							:label = 'i[1]'
							:value = 'i[0]'
						/>
					</var-select>
					<var-select placeholder = '类型' multiple v-model = 'card.type'>
						<var-option
							v-for = 'i in config.types'
							:label = 'i[1]'
							:value = 'i[0]'
						/>
					</var-select>
				</var-cell>
				<br>
				<var-cell>
					<var-select placeholder = '效果分类' multiple v-model = 'card.category'>
						<var-option
							v-for = 'i in config.category'
							:label = 'i[1]'
							:value = 'i[0]'
						/>
					</var-select>
				</var-cell>
				<br/>
				<var-input
					placeholder = '描述'
					variant = 'outlined'
					textarea v-model = 'card.desc'
				/>
				<div class = 'hint no-scrollbar'>
					<var-input
						v-for = '(_, v) in new Array(16)'
						:placeholder = '`提示文本${v + 1}`'
						v-model = 'card.hint[v]'
					/>
				</div>
			</div>
		</div>
	</div>
</template>
<script setup lang = 'ts'>
	import { onBeforeMount, reactive } from 'vue';
	import { convertFileSrc } from '@tauri-apps/api/core';

	import invoke from '../script/invoke';
	import config from '../script/config';

	const href = window.location.href;

	const arr = {
		pic : [
			[
				{ link: 0x40, url: [`${href}/arrow-1-off.png`, `${href}/arrow-1-on.png`] },
				{ link: 0x80, url: [`${href}/arrow-2-off.png`, `${href}/arrow-2-on.png`] },
				{ link: 0x100, url: [`${href}/arrow-3-off.png`, `${href}/arrow-3-on.png`] },
				{ link: 0x8, url: [`${href}/arrow-4-off.png`, `${href}/arrow-4-on.png`] }
			], [
				{ link: 0x20, url: [`${href}/arrow-5-off.png`, `${href}/arrow-5-on.png`] },
				{ link: 0x1, url: [`${href}/arrow-6-off.png`, `${href}/arrow-6-on.png`] },
				{ link: 0x2, url: [`${href}/arrow-7-off.png`, `${href}/arrow-7-on.png`] },
				{ link: 0x4, url: [`${href}/arrow-8-off.png`, `${href}/arrow-8-on.png`] }
			]
		]
	};

	const card = reactive({
		id : '',
		name : '',
		ot : [] as Array<number>,
		alias : '',
		level : '',
		scale : '',
		atk : '',
		def : '',
		link : 0,
		type : [] as Array<number>,
		race : 0,
		attribute : 0,
		category : [] as Array<number>,
		setcode : ['', '', '', ''] as [string, string, string, string],
		desc : '',
		hint : [] as Array<string>,
		pic : '',
		update : {
			link : (link : number) => card.link+= (card.link & link ? - 1 : 1) * link
		}
	});

	const to_array = (flags : number) => {
		const result = [];
		for (let i = 0; i < 32; i++)
			if (flags & (1 << i))
				result.push(1 << i);
		return result;
	};

	const props = defineProps<{
		db : string;
		code : number;
	}>();

	onBeforeMount(async () => {
		const [datas, texts] = await invoke.get_db(props.db, props.code);
		card.id = datas[0].toString();
		card.ot = to_array(datas[1]);
		card.alias = datas[2].toString();
		card.setcode = (datas[3]
			.toString(16)
			.padStart(16, '0')
			.match(/.{1,4}/g)
			?.map(i => parseInt(i, 16).toString(16)) ?? ['', '', '', '']).reverse() as [string, string, string, string];
		card.type = to_array(datas[4]);
		card.atk = datas[5].toString();
		card.def = datas[6].toString();
		card.link = card.type.includes(0x4000000) ? datas[6] : 0;
		const level = datas[7].toString(16).padStart(7, '0')
		card.level = (parseInt(level.slice(-4), 16) | 0).toString();
		card.scale = (parseInt(level.slice(-6, -4), 16) | 0).toString();
		card.race = datas[8];
		card.attribute = datas[9];
		card.category = to_array(datas[10]);
		card.name = texts[0];
		card.desc = texts[1];
		card.hint = texts.slice(2);
		const arr = props.db.split(/[\\/]/);
		card.pic = convertFileSrc(arr.slice(0, -1).join('/') + '/pics/' + props.code + '.jpg');
	});
</script>
<style scoped lang = 'scss'>
	.card {
		> div {
			padding-top: 10px;
			width: 100%;
			height: calc(100% - 60px);
			display: flex;
			flex-wrap: wrap;
			overflow-y: auto;
			.var-cell {
				height: 50px;
			}
			> div {
				width: 50%;
			}
			> div:first-of-type {
				> div:nth-child(3) {
					position: relative;
					transition: all 0.1s ease;
					width: 100%;
					height: 240px;
					> img {
						position: absolute;
						width: 160px;
						left: 50%;
						transform: translateX(calc(-50%));
						transition: all 0.1s ease;
					}
					> div {
						position: absolute;
						width: 180px;
						height: 180px;
						display: grid;
						grid-template-rows: repeat(3, 1fr);
						grid-template-columns: repeat(3, 1fr);
						left: 50%;
						transform: translateX(-50%);
						transition: all 0.1s ease;
						> img {
							width: 60px;
							height: 60px;
						}
					}
				}
				> div:nth-child(4),
				> div:nth-child(5),
				> div:nth-child(6),
				> div:nth-child(7),
				> div:nth-child(8) {
					:deep(.var-cell__content) {
						display: flex;
						gap: 10px;
						.var-select, .var-input {
							width: calc(50% - 5px);
						}
					}
				}
			}
			> div:nth-child(2) {
				> div:first-of-type,
				> div:nth-child(3) {
					:deep(.var-cell__content) {
						display: flex;
						gap: 10px;
						.var-select {
							width: calc(50% - 5px);
						}
					}
				}
				.hint {
					overflow-y: auto;
					height: 220px;
				}
			}
			@media (max-aspect-ratio: 1/1) {
				gap: 10px;
				transform: translateX(-3px);
				> div {
					width: 100% !important;
				}
				> div:first-of-type{
					> div:nth-child(2) {
						position: relative;
						width: 100%;
						height: 150px;
						> img {
							width: 100px;
							transform: translateX(calc(-50% - 5px));
						}
						> div {
							width: 120px;
							height: 120px;
							> img {
								width: 30px;
								height: 30px;
							}
						}
					}
				}
			}
		}
	}
</style>