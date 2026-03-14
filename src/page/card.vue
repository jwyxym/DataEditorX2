<template>
	<div class = 'card'>
		<var-app-bar :title = 'card.name'>
			<template #left>
				<var-button
					color = 'transparent'
					text-color = '#fff'
					round
					text
					@click = 'card.exit'
				>
					<var-icon name = 'chevron-left' :size = '24' />
				</var-button>
			</template>
			<template #right>
				<var-menu>
					<var-button
						color = 'transparent'
						text-color = '#fff'
						round
						text
					>
						<var-icon name = 'menu' :size = '24' />
					</var-button>
					<template #menu>
						<var-cell ripple class = 'pointer' @click = 'card.save'>保存</var-cell>
						<var-cell ripple class = 'pointer' @click = 'card.coder'>脚本</var-cell>
						<transition name = 'opacity'>
							<var-cell ripple class = 'pointer' v-show = '!page.coder' @click = 'card.del'>删除</var-cell>
						</transition>
					</template>
				</var-menu>
			</template>
		</var-app-bar>
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
						v-show = '!card.type.includes(0x4000000)'
						placeholder = '守备力'
						v-model = 'card.def'
						type = 'number'
					/>
				</var-cell>
				<var-cell>
					<var-input
						placeholder = '等级/阶级/连接数'
						v-model = 'card.level'
						type = 'number'
					/>
					<var-input
						v-show = 'card.type.includes(0x1000000)'
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
		<transition name = 'move_out'>
			<Coder v-if = 'page.coder' :lua = 'card.lua[1]' :path = 'card.lua[0]' class = 'coder'/>
		</transition>
		<div class = 'btns'>
			<var-chip plain type = 'success' @click = 'card.exit'>退出</var-chip>
			<var-chip plain type = 'success' @click = 'card.save'>保存</var-chip>
			<transition name = 'opacity'>
				<var-chip plain type = 'success' v-show = '!page.coder' @click = 'card.coder'>脚本</var-chip>
			</transition>
			<transition name = 'opacity'>
				<var-chip plain type = 'danger' v-show = '!page.coder' @click = 'card.del'>删除</var-chip>
			</transition>
		</div>
	</div>
</template>
<script setup lang = 'ts'>
	import { onBeforeMount, reactive } from 'vue';
	import { convertFileSrc } from '@tauri-apps/api/core';

	import invoke from '../script/invoke';
	import config from '../script/config';
	import toast from '../script/toast';
	import emitter from '../script/emit';

	import Coder from './coder.vue';

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

	const page = reactive({
		coder : false
	})

	const card = reactive({
		lua : ['' ,'local s,id,o=GetID()\nfunction s.initial_effect(c)\n\nend'] as [string, string],
		id : '',
		old_name : '',
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
		},
		coder : () => {
			parseInt(card.id) ? page.coder = !page.coder
				: toast.error('需要卡号');
		},
		save : async () => {
			if (page.coder) {
				emitter.emit('save');
				return;
			}
			if (!parseInt(card.id)) {
				toast.error('需要卡号');
				return;
			}
			const row = [
				parseInt(card.id),
				card.ot.reduce((acc, curr) => acc + curr, 0),
				parseInt(card.alias),
				card.setcode.map(i => parseInt(i, 16)).reduce((acc, curr) => acc + curr, 0),
				card.type.reduce((acc, curr) => acc + curr, 0),
				parseInt(card.atk),
				card.type.includes(0x4000000) ? card.link : parseInt(card.def),
				parseInt(card.level),
				card.race,
				card.attribute,
				card.category.reduce((acc, curr) => acc + curr, 0)
			];
			if (row.findIndex(i => isNaN(i)) > -1) {
				toast.error('参数错误');
				return;
			}

			if (await invoke.write_db(props.db, props.code, [
				row,
				[
					card.name,
					card.desc
				].concat(card.hint)
			])) {
				emitter.emit('change', [props.code, parseInt(card.id), card.name]);
				card.old_name = card.name;
				const arr = props.db.split(/[\\/]/);
				card.lua[0] = arr.slice(0, -1).join('/') + `c${card.id}.lua`;
				toast.info('保存成功');
			}
		},
		del : async () => {
			if (await invoke.del_db(props.db, props.code)) {
				emitter.emit('change', [props.code, null, null]);
				toast.info('删除成功');
			}
		},
		exit : () => {
			page.coder ? page.coder = false
				: emitter.emit('exit');
		},
	});

	const to_array = (flags : number) => {
		const result = [];
		for (let i = 0; i < 32; i++) {
			const bit = 1 << i;
			if (flags & bit)
				result.push(bit >>> 0);
		}
		return result;
	};

	const props = defineProps<{
		db : string;
		code : number;
	}>();

	onBeforeMount(async () => {
		if (props.code === - 2) {
			card.pic = href + 'cover.jpg';
			return;
		}
		if (props.code === 0) {
			const arr = props.db.split(/[\\/]/);
			card.lua[0] = arr.slice(0, -1).join('/') + `c${props.code}.lua`;
			card.pic = href + 'cover.jpg';
			card.id = '0';
			card.alias = '0';
			card.level = '0';
			card.scale = '0';
			card.atk = '0';
			card.def = '0';
			card.setcode = ['0', '0', '0', '0'];
			return;
		}
		const c = await invoke.get_db(props.db, props.code);
		if (!c) return;
		card.pic = c.path.length > 0 ? convertFileSrc(c.path)
			: (href + 'cover.jpg');
		const [datas, texts] = c.card;
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
		card.old_name = texts[0];
		card.name = texts[0];
		card.desc = texts[1];
		card.hint = texts.slice(2);
		card.lua = [
			c.lua[0],
			c.lua[1].length > 0 ? c.lua[1] : 'local s,id,o=GetID()\nfunction s.initial_effect(c)\n\nend'
		];
	});
</script>
<style scoped lang = 'scss'>
	.card {
		:deep(.var-app-bar) {
			width: 100%;
			height: 0px;
			opacity: 0;
			transition: all 0.1s ease;
			user-select: none;
			pointer-events: none;
		}
		.btns {
			width: 100%;
			height: 50px;
			display: flex;
			align-items: center;
			justify-content: center;
			gap: 5px;
			transition: all 0.1s ease;
		}
		> div:nth-child(2) {
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
		}
		.coder {
			position: absolute;
			left: 0;
			top: 0;
			width: 100%;
			height: calc(100% - 50px);
		}
		
		@media (max-aspect-ratio: 1/1) {
			:deep(.var-app-bar) {
				height: 50px;
				opacity: 1;
				user-select: initial;
				pointer-events: initial;
			}
			.btns {
				height: 0px !important;
				opacity: 0;
			}
			> div:nth-child(2) {
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
				.hint {
					height: initial !important;
					display: flex;
					flex-direction: column;
					align-items: center;
					.var-input {
						width: 80%;
					}
				}
			}
			.coder {
				top: 50px;
				height: 100%;
			}
		}
	}
	.move_out {
		&-enter-active,
		&-leave-active {
			transition: all 0.1s ease;
		}

		&-enter-from,
		&-leave-to {
			transform: translate(-100%);
		}

		&-enter-to,
		&-leave-from {
			transform: translate(0);
		}
	}
	.opacity {
		&-enter-active,
		&-leave-active {
			transition: opacity 0.1s ease;
		}

		&-enter-from,
		&-leave-to {
			opacity: 0;
		}

		&-enter-to,
		&-leave-from {
			opacity: 1;
		}
	}
</style>