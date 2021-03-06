'use strict';

var UIColorPicker = (function UIColorPicker() {

	var subscribers: any = [];
	var pickers = [];

	/**
	 * RGBA Color class
	 *
	 * HSV/HSB and HSL (hue, saturation, value / brightness, lightness)
	 * @param hue			0-360
	 * @param saturation	0-100
	 * @param value 		0-100
	 * @param lightness		0-100
	 */

	function Color(color: any = {}) {

		if (color instanceof Color === true) {
			this.copy(color);
			return;
		}

		return {
			r: 0,
			g: 0,
			b: 0,
			a: 1,
			hue: 0,
			saturation: 0,
			value: 0,
			lightness: 0,
			format: 'HSV',
		} as any;
		// this.r = 0;
		// this.g = 0;
		// this.b = 0;
		// this.a = 1;
		// this.hue = 0;
		// this.saturation = 0;
		// this.value = 0;
		// this.lightness = 0;
		// this.format = 'HSV';
	}

	Color.prototype.copy = function copy(obj: any) {
		if (obj instanceof Color !== true) {
			console.log('Typeof parameter not Color');
			return;
		}

		this.r = obj.r;
		this.g = obj.g;
		this.b = obj.b;
		this.a = obj.a;
		this.hue = obj.hue;
		this.saturation = obj.saturation;
		this.value = obj.value;
		this.format = '' + obj.format;
		this.lightness = obj.lightness;
	};

	Color.prototype.setFormat = function setFormat(format: string) {
		if (format === 'HSV')
			this.format = 'HSV';
		if (format === 'HSL')
			this.format = 'HSL';
	};

	/*========== Methods to set Color Properties ==========*/

	Color.prototype.isValidRGBValue = function isValidRGBValue(value: number) {
		return (typeof (value) === 'number' && isNaN(value) === false &&
			value >= 0 && value <= 255);
	};

	Color.prototype.setRGBA = function setRGBA(red: number, green: number, blue: number, alpha: number) {
		if (this.isValidRGBValue(red) === false ||
			this.isValidRGBValue(green) === false ||
			this.isValidRGBValue(blue) === false)
			return;

		this.r = red | 0;
		this.g = green | 0;
		this.b = blue | 0;

		if (this.isValidRGBValue(alpha) === true)
			this.a = alpha | 0;
	};

	Color.prototype.setByName = function setByName(name: string, value: number) {
		if (name === 'r' || name === 'g' || name === 'b') {
			if (this.isValidRGBValue(value) === false)
				return;

			this[name] = value;
			this.updateHSX();
		}
	};

	Color.prototype.setHSV = function setHSV(hue: number, saturation: number, value: number) {
		this.hue = hue;
		this.saturation = saturation;
		this.value = value;
		this.HSVtoRGB();
	};

	Color.prototype.setHSL = function setHSL(hue: number, saturation: number, lightness: number) {
		this.hue = hue;
		this.saturation = saturation;
		this.lightness = lightness;
		this.HSLtoRGB();
	};

	Color.prototype.setHue = function setHue(value: number) {
		if (typeof (value) !== 'number' || isNaN(value) === true ||
			value < 0 || value > 359)
			return;
		this.hue = value;
		this.updateRGB();
	};

	Color.prototype.setSaturation = function setSaturation(value: number) {
		if (typeof (value) !== 'number' || isNaN(value) === true ||
			value < 0 || value > 100)
			return;
		this.saturation = value;
		this.updateRGB();
	};

	Color.prototype.setValue = function setValue(value: number) {
		if (typeof (value) !== 'number' || isNaN(value) === true ||
			value < 0 || value > 100)
			return;
		this.value = value;
		this.HSVtoRGB();
	};

	Color.prototype.setLightness = function setLightness(value: number) {
		if (typeof (value) !== 'number' || isNaN(value) === true ||
			value < 0 || value > 100)
			return;
		this.lightness = value;
		this.HSLtoRGB();
	};

	Color.prototype.setHexa = function setHexa(value: any) {
		var valid = /(^#{0,1}[0-9A-F]{6}$)|(^#{0,1}[0-9A-F]{3}$)/i.test(value);

		if (valid !== true)
			return;

		if (value[0] === '#')
			value = value.slice(1, value.length);

		if (value.length === 3)
			value = value.replace(/([0-9A-F])([0-9A-F])([0-9A-F])/i, '$1$1$2$2$3$3');

		this.r = parseInt(value.substr(0, 2), 16);
		this.g = parseInt(value.substr(2, 2), 16);
		this.b = parseInt(value.substr(4, 2), 16);

		this.alpha = 1;
		this.RGBtoHSV();
	};

	/*========== Conversion Methods ==========*/

	Color.prototype.convertToHSL = function convertToHSL() {
		if (this.format === 'HSL')
			return;

		this.setFormat('HSL');
		this.RGBtoHSL();
	};

	Color.prototype.convertToHSV = function convertToHSV() {
		if (this.format === 'HSV')
			return;

		this.setFormat('HSV');
		this.RGBtoHSV();
	};

	/*========== Update Methods ==========*/

	Color.prototype.updateRGB = function updateRGB() {
		if (this.format === 'HSV') {
			this.HSVtoRGB();
			return;
		}

		if (this.format === 'HSL') {
			this.HSLtoRGB();
			return;
		}
	};

	Color.prototype.updateHSX = function updateHSX() {
		if (this.format === 'HSV') {
			this.RGBtoHSV();
			return;
		}

		if (this.format === 'HSL') {
			this.RGBtoHSL();
			return;
		}
	};

	Color.prototype.HSVtoRGB = function HSVtoRGB() {
		var sat = this.saturation / 100;
		var value = this.value / 100;
		var C = sat * value;
		var H = this.hue / 60;
		var X = C * (1 - Math.abs(H % 2 - 1));
		var m = value - C;
		var precision = 255;

		C = (C + m) * precision | 0;
		X = (X + m) * precision | 0;
		m = m * precision | 0;

		if (H >= 0 && H < 1) { this.setRGBA(C, X, m); return; }
		if (H >= 1 && H < 2) { this.setRGBA(X, C, m); return; }
		if (H >= 2 && H < 3) { this.setRGBA(m, C, X); return; }
		if (H >= 3 && H < 4) { this.setRGBA(m, X, C); return; }
		if (H >= 4 && H < 5) { this.setRGBA(X, m, C); return; }
		if (H >= 5 && H < 6) { this.setRGBA(C, m, X); return; }
	};

	Color.prototype.HSLtoRGB = function HSLtoRGB() {
		var sat = this.saturation / 100;
		var light = this.lightness / 100;
		var C = sat * (1 - Math.abs(2 * light - 1));
		var H = this.hue / 60;
		var X = C * (1 - Math.abs(H % 2 - 1));
		var m = light - C / 2;
		var precision = 255;

		C = (C + m) * precision | 0;
		X = (X + m) * precision | 0;
		m = m * precision | 0;

		if (H >= 0 && H < 1) { this.setRGBA(C, X, m); return; }
		if (H >= 1 && H < 2) { this.setRGBA(X, C, m); return; }
		if (H >= 2 && H < 3) { this.setRGBA(m, C, X); return; }
		if (H >= 3 && H < 4) { this.setRGBA(m, X, C); return; }
		if (H >= 4 && H < 5) { this.setRGBA(X, m, C); return; }
		if (H >= 5 && H < 6) { this.setRGBA(C, m, X); return; }
	};

	Color.prototype.RGBtoHSV = function RGBtoHSV() {
		var red = this.r / 255;
		var green = this.g / 255;
		var blue = this.b / 255;

		var cmax = Math.max(red, green, blue);
		var cmin = Math.min(red, green, blue);
		var delta = cmax - cmin;
		var hue = 0;
		var saturation = 0;

		if (delta) {
			if (cmax === red) { hue = ((green - blue) / delta); }
			if (cmax === green) { hue = 2 + (blue - red) / delta; }
			if (cmax === blue) { hue = 4 + (red - green) / delta; }
			if (cmax) saturation = delta / cmax;
		}

		this.hue = 60 * hue | 0;
		if (this.hue < 0) this.hue += 360;
		this.saturation = (saturation * 100) | 0;
		this.value = (cmax * 100) | 0;
	};

	Color.prototype.RGBtoHSL = function RGBtoHSL() {
		var red = this.r / 255;
		var green = this.g / 255;
		var blue = this.b / 255;

		var cmax = Math.max(red, green, blue);
		var cmin = Math.min(red, green, blue);
		var delta = cmax - cmin;
		var hue = 0;
		var saturation = 0;
		var lightness = (cmax + cmin) / 2;
		var X = (1 - Math.abs(2 * lightness - 1));

		if (delta) {
			if (cmax === red) { hue = ((green - blue) / delta); }
			if (cmax === green) { hue = 2 + (blue - red) / delta; }
			if (cmax === blue) { hue = 4 + (red - green) / delta; }
			if (cmax) saturation = delta / X;
		}

		this.hue = 60 * hue | 0;
		if (this.hue < 0) this.hue += 360;
		this.saturation = (saturation * 100) | 0;
		this.lightness = (lightness * 100) | 0;
	};

	/*========== Get Methods ==========*/

	Color.prototype.getHexa = function getHexa() {
		var r = this.r.toString(16);
		var g = this.g.toString(16);
		var b = this.b.toString(16);
		if (this.r < 16) r = '0' + r;
		if (this.g < 16) g = '0' + g;
		if (this.b < 16) b = '0' + b;
		var value = '#' + r + g + b;
		return value.toUpperCase();
	};

	Color.prototype.getRGBA = function getRGBA() {

		var rgb = '(' + this.r + ', ' + this.g + ', ' + this.b;
		var a = '';
		var v = '';
		var x = parseFloat(this.a);
		if (x !== 1) {
			a = 'a';
			v = ', ' + x;
		}

		var value = 'rgb' + a + rgb + v + ')';
		return value;
	};

	Color.prototype.getHSLA = function getHSLA() {
		if (this.format === 'HSV') {
			var color: any = Color(this);
			color.setFormat('HSL');
			color.updateHSX();
			return color.getHSLA();
		}

		var a = '';
		var v = '';
		var hsl = '(' + this.hue + ', ' + this.saturation + '%, ' + this.lightness + '%';
		var x = parseFloat(this.a);
		if (x !== 1) {
			a = 'a';
			v = ', ' + x;
		}

		var value = 'hsl' + a + hsl + v + ')';
		return value;
	};

	Color.prototype.getColor = function getColor() {
		if (this.a)
			return this.getHexa();
		return this.getRGBA();
	};

	/*=======================================================================*/
	/*=======================================================================*/

	/*========== Capture Mouse Movement ==========*/

	var setMouseTracking = function setMouseTracking(elem: Element, callback: any) {
		elem.addEventListener('mousedown', function (e) {
			callback(e);
			document.addEventListener('mousemove', callback);
		});

		document.addEventListener('mouseup', function (e) {
			document.removeEventListener('mousemove', callback);
		});
	};

	/*====================*/
	// Color Picker Class
	/*====================*/

	function ColorPicker(node: any) {
		this.color = Color();
		this.node = node;
		this.subscribers = [];

		var type = this.node.getAttribute('data-mode');
		var topic = this.node.getAttribute('data-topic');

		this.topic = topic;
		this.picker_mode = (type === 'HSL') ? 'HSL' : 'HSV';
		this.color.setFormat(this.picker_mode);

		this.createPickingArea();
		this.createHueArea();

		this.newInputComponent('H', 'hue', this.inputChangeHue.bind(this));
		this.newInputComponent('S', 'saturation', this.inputChangeSaturation.bind(this));
		this.newInputComponent('V', 'value', this.inputChangeValue.bind(this));
		this.newInputComponent('L', 'lightness', this.inputChangeLightness.bind(this));

		this.createAlphaArea();

		this.newInputComponent('R', 'red', this.inputChangeRed.bind(this));
		this.newInputComponent('G', 'green', this.inputChangeGreen.bind(this));
		this.newInputComponent('B', 'blue', this.inputChangeBlue.bind(this));

		this.createPreviewBox();
		this.createChangeModeButton();

		this.newInputComponent('alpha', 'alpha', this.inputChangeAlpha.bind(this));
		this.newInputComponent('hexa', 'hexa', this.inputChangeHexa.bind(this));

		this.setColor(this.color);
		pickers[topic] = this;
	}

	/*************************************************************************/
	//				Function for generating the color-picker
	/*************************************************************************/

	ColorPicker.prototype.createPickingArea = function createPickingArea() {
		var area = document.createElement('div');
		var picker = document.createElement('div');

		area.className = 'picking-area';
		picker.className = 'picker';

		this.picking_area = area;
		this.color_picker = picker;
		setMouseTracking(area, this.updateColor.bind(this));

		area.appendChild(picker);
		this.node.appendChild(area);
		console.log(this.picking_area.offsetLeft, this.picking_area.offsetTop);
	};

	ColorPicker.prototype.createHueArea = function createHueArea() {
		var area = document.createElement('div');
		var picker = document.createElement('div');

		area.className = 'hue';
		picker.className = 'slider-picker';

		this.hue_area = area;
		this.hue_picker = picker;
		setMouseTracking(area, this.updateHueSlider.bind(this));

		area.appendChild(picker);
		this.node.appendChild(area);
	};

	ColorPicker.prototype.createAlphaArea = function createAlphaArea() {
		var area = document.createElement('div');
		var mask = document.createElement('div');
		var picker = document.createElement('div');

		area.className = 'alpha';
		mask.className = 'alpha-mask';
		picker.className = 'slider-picker';

		this.alpha_area = area;
		this.alpha_mask = mask;
		this.alpha_picker = picker;
		setMouseTracking(area, this.updateAlphaSlider.bind(this));

		area.appendChild(mask);
		mask.appendChild(picker);
		this.node.appendChild(area);
	};

	ColorPicker.prototype.createPreviewBox = function createPreviewBox(e: Event) {
		var preview_box = document.createElement('div');
		var preview_color = document.createElement('div');

		preview_box.className = 'preview';
		preview_color.className = 'preview-color';

		this.preview_color = preview_color;

		preview_box.appendChild(preview_color);
		this.node.appendChild(preview_box);
	};

	ColorPicker.prototype.newInputComponent = function newInputComponent(title: string, topic: string, onChangeFunc: any) {
		var wrapper = document.createElement('div');
		var input = document.createElement('input');
		var info = document.createElement('span');

		wrapper.className = 'input';
		wrapper.setAttribute('data-topic', topic);
		info.textContent = title;
		info.className = 'name';
		input.setAttribute('type', 'text');
		input.id = `color-picker-${title}`

		wrapper.appendChild(info);
		wrapper.appendChild(input);
		this.node.appendChild(wrapper);

		input.addEventListener('change', onChangeFunc);
		input.addEventListener('click', function () {
			this.select();
		});

		this.subscribe(topic, function (value: any) {
			input.value = value;
		});
	};

	ColorPicker.prototype.createChangeModeButton = function createChangeModeButton() {

		var button = document.createElement('div');
		button.className = 'switch_mode';
		button.addEventListener('click', function () {
			if (this.picker_mode === 'HSV')
				this.setPickerMode('HSL');
			else
				this.setPickerMode('HSV');

		}.bind(this));

		this.node.appendChild(button);
	};

	/*************************************************************************/
	//					Updates properties of UI elements
	/*************************************************************************/

	ColorPicker.prototype.updateColor = function updateColor(e: MouseEvent) {
		var x = e.pageX - this.picking_area.offsetLeft;
		var y = e.pageY - this.picking_area.offsetTop;
		console.log(this.picking_area.offsetLeft, this.picking_area.offsetTop)
		// var x = e.pageX;
		// var y = e.pageY;
		console.log('update color');
		console.log(x, y);
		var picker_offset = 5;

		// width and height should be the same
		var size = this.picking_area.clientWidth;

		if (x > size) x = size;
		if (y > size) y = size;
		if (x < 0) x = 0;
		if (y < 0) y = 0;

		var value = 100 - (y * 100 / size) | 0;
		var saturation = x * 100 / size | 0;

		if (this.picker_mode === 'HSV')
			this.color.setHSV(this.color.hue, saturation, value);
		if (this.picker_mode === 'HSL')
			this.color.setHSL(this.color.hue, saturation, value);

		this.color_picker.style.left = x - picker_offset + 'px';
		this.color_picker.style.top = y - picker_offset + 'px';

		this.updateAlphaGradient();
		this.updatePreviewColor();

		this.notify('value', value);
		this.notify('lightness', value);
		this.notify('saturation', saturation);

		this.notify('red', this.color.r);
		this.notify('green', this.color.g);
		this.notify('blue', this.color.b);
		this.notify('hexa', this.color.getHexa());

		notify(this.topic, this.color);
	};

	ColorPicker.prototype.updateHueSlider = function updateHueSlider(e: MouseEvent) {
		var x = e.pageX - this.hue_area.offsetLeft;
		var width = this.hue_area.clientWidth;

		if (x < 0) x = 0;
		if (x > width) x = width;

		// TODO 360 => 359
		var hue = ((359 * x) / width) | 0;
		// if (hue === 360) hue = 359;

		this.updateSliderPosition(this.hue_picker, x);
		this.setHue(hue);
	};

	ColorPicker.prototype.updateAlphaSlider = function updateAlphaSlider(e: MouseEvent) {
		var x = e.pageX - this.alpha_area.offsetLeft;
		var width = this.alpha_area.clientWidth;

		if (x < 0) x = 0;
		if (x > width) x = width;

		this.color.a = (x / width).toFixed(2);

		this.updateSliderPosition(this.alpha_picker, x);
		this.updatePreviewColor();

		this.notify('alpha', this.color.a);
		notify(this.topic, this.color);
	};

	ColorPicker.prototype.setHue = function setHue(value: MouseEvent) {
		this.color.setHue(value);

		this.updatePickerBackground();
		this.updateAlphaGradient();
		this.updatePreviewColor();

		this.notify('red', this.color.r);
		this.notify('green', this.color.g);
		this.notify('blue', this.color.b);
		this.notify('hexa', this.color.getHexa());
		this.notify('hue', this.color.hue);

		notify(this.topic, this.color);
	};

	// Updates when one of Saturation/Value/Lightness changes
	ColorPicker.prototype.updateSLV = function updateSLV() {
		this.updatePickerPosition();
		this.updateAlphaGradient();
		this.updatePreviewColor();

		this.notify('red', this.color.r);
		this.notify('green', this.color.g);
		this.notify('blue', this.color.b);
		this.notify('hexa', this.color.getHexa());

		notify(this.topic, this.color);
	};

	/*************************************************************************/
	//				Update positions of various UI elements
	/*************************************************************************/

	ColorPicker.prototype.updatePickerPosition = function updatePickerPosition() {
		var size = this.picking_area.clientWidth;
		var value = 0;
		var offset = 5;

		if (this.picker_mode === 'HSV')
			value = this.color.value;
		if (this.picker_mode === 'HSL')
			value = this.color.lightness;

		var x = (this.color.saturation * size / 100) | 0;
		var y = size - (value * size / 100) | 0;

		this.color_picker.style.left = x - offset + 'px';
		this.color_picker.style.top = y - offset + 'px';
	};

	ColorPicker.prototype.updateSliderPosition = function updateSliderPosition(elem: HTMLElement, pos: number) {
		elem.style.left = Math.max(pos - 3, -2) + 'px';
	};

	ColorPicker.prototype.updateHuePicker = function updateHuePicker() {
		var size = this.hue_area.clientWidth;
		var offset = 1;
		var pos = (this.color.hue * size / 360) | 0;
		this.hue_picker.style.left = pos - offset + 'px';
	};

	ColorPicker.prototype.updateAlphaPicker = function updateAlphaPicker() {
		var size = this.alpha_area.clientWidth;
		var offset = 1;
		var pos = (this.color.a * size) | 0;
		this.alpha_picker.style.left = pos - offset + 'px';
	};

	/*************************************************************************/
	//						Update background colors
	/*************************************************************************/

	ColorPicker.prototype.updatePickerBackground = function updatePickerBackground() {
		var nc: any = Color(this.color);
		nc.setHSV(nc.hue, 100, 100);
		this.picking_area.style.backgroundColor = nc.getHexa();
	};

	ColorPicker.prototype.updateAlphaGradient = function updateAlphaGradient() {
		this.alpha_mask.style.backgroundColor = this.color.getHexa();
	};

	ColorPicker.prototype.updatePreviewColor = function updatePreviewColor() {
		this.preview_color.style.backgroundColor = this.color.getColor();
	};

	/*************************************************************************/
	//						Update input elements
	/*************************************************************************/

	ColorPicker.prototype.inputChangeHue = function inputChangeHue(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.setHue(value);
		this.updateHuePicker();
	};

	ColorPicker.prototype.inputChangeSaturation = function inputChangeSaturation(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.color.setSaturation(value);
		(e.target as HTMLInputElement).value = this.color.saturation;
		this.updateSLV();
	};

	ColorPicker.prototype.inputChangeValue = function inputChangeValue(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.color.setValue(value);
		(e.target as HTMLInputElement).value = this.color.value;
		this.updateSLV();
	};

	ColorPicker.prototype.inputChangeLightness = function inputChangeLightness(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.color.setLightness(value);
		(e.target as HTMLInputElement).value = this.color.lightness;
		this.updateSLV();
	};

	ColorPicker.prototype.inputChangeRed = function inputChangeRed(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.color.setByName('r', value);
		(e.target as HTMLInputElement).value = this.color.r;
		this.setColor(this.color);
	};

	ColorPicker.prototype.inputChangeGreen = function inputChangeGreen(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.color.setByName('g', value);
		(e.target as HTMLInputElement).value = this.color.g;
		this.setColor(this.color);
	};

	ColorPicker.prototype.inputChangeBlue = function inputChangeBlue(e: InputEvent) {
		var value = parseInt((e.target as HTMLInputElement).value);
		this.color.setByName('b', value);
		(e.target as HTMLInputElement).value = this.color.b;
		this.setColor(this.color);
	};

	ColorPicker.prototype.inputChangeAlpha = function inputChangeAlpha(e: InputEvent) {
		var value = parseFloat((e.target as HTMLInputElement).value);

		if (typeof value === 'number' && isNaN(value) === false &&
			value >= 0 && value <= 1)
			this.color.a = value.toFixed(2);

		(e.target as HTMLInputElement).value = this.color.a;
		this.updateAlphaPicker();
	};

	ColorPicker.prototype.inputChangeHexa = function inputChangeHexa(e: InputEvent) {
		var value = (e.target as HTMLInputElement).value;
		this.color.setHexa(value);
		this.setColor(this.color);
	};

	/*************************************************************************/
	//							Internal Pub/Sub
	/*************************************************************************/

	ColorPicker.prototype.subscribe = function subscribe(topic: string, callback: any) {
		this.subscribers[topic] = callback;
	};

	ColorPicker.prototype.notify = function notify(topic: string, value: any) {
		if (this.subscribers[topic])
			this.subscribers[topic](value);
	};

	/*************************************************************************/
	//							Set Picker Properties
	/*************************************************************************/

	ColorPicker.prototype.setColor = function setColor(color: any) {
		if (color instanceof Color !== true) {
			console.log('Typeof parameter not Color');
			return;
		}

		if (color.format !== this.picker_mode) {
			color.setFormat(this.picker_mode);
			color.updateHSX();
		}

		this.color.copy(color);
		this.updateHuePicker();
		this.updatePickerPosition();
		this.updatePickerBackground();
		this.updateAlphaPicker();
		this.updateAlphaGradient();
		this.updatePreviewColor();

		this.notify('red', this.color.r);
		this.notify('green', this.color.g);
		this.notify('blue', this.color.b);

		this.notify('hue', this.color.hue);
		this.notify('saturation', this.color.saturation);
		this.notify('value', this.color.value);
		this.notify('lightness', this.color.lightness);

		this.notify('alpha', this.color.a);
		this.notify('hexa', this.color.getHexa());
		notify(this.topic, this.color);
	};

	ColorPicker.prototype.setPickerMode = function setPickerMode(mode: string) {
		if (mode !== 'HSV' && mode !== 'HSL')
			return;

		this.picker_mode = mode;
		this.node.setAttribute('data-mode', this.picker_mode);
		this.setColor(this.color);
	};

	/*************************************************************************/
	//								UNUSED
	/*************************************************************************/

	var notify = function notify(topic: string, value: number) {
		if (subscribers[topic] === undefined || subscribers[topic].length === 0)
			return;

		var color = Color(value);
		for (var i in subscribers[topic])
			subscribers[topic][i](color);
	};

	var init = function init() {
		var elem = document.querySelectorAll('.ui-color-picker');
		var size = elem.length;
		for (var i = 0; i < size; i++)
			ColorPicker(elem[i]);
	};

	return {
		init: init,
		Color: Color,
	};

})();

window.addEventListener("load", function () {
	console.log('here');
	ColorPickerTool.init();
});

var ColorPickerTool = (function ColorPickerTool() {

	var init = function init() {
		UIColorPicker.init();
	};

	return {
		init: init
	};

})();

export default ColorPickerTool;
