document.addEventListener("DOMContentLoaded", () => {
	console.log("🧠 DOM ready");
	const form = document.getElementById("waveformForm");
	const img = document.getElementById("preview");
	const submitButton = document.getElementById("submitButton");

	form.addEventListener("submit", async (e) => {
		e.preventDefault();
		img.classList.add('hidden');
		submitButton.disabled = true;
		submitButton.classList.add("opacity-50", "cursor-not-allowed");

		const data = new FormData(form);
		const params = new URLSearchParams(data).toString();

		try {
			const res = await fetch("/lissajous?" + params);
			if (!res.ok) {
				alert("Failed to generate waveform.");
				return;
			} else {
				console.log("Fetched waveform image from server successfully");
			}
			const blob = await res.blob();
			img.src = URL.createObjectURL(blob);
			img.classList.remove('hidden');
		} catch (err) {
			alert("Error: " + err.message);
		} finally {
			submitButton.disabled = false;
			submitButton.classList.remove("opacity-50", "cursor-not-allowed");
		}
	});

	const fgInput = document.querySelector('input[name="fgColor"]');
	const bgInput = document.querySelector('input[name="bgColor"]');

	const fgPicker = Pickr.create({
		el: '#fgPicker',
		theme: 'monolith',
		default: fgInput.value,
		components: {
			preview: true,
			opacity: true,
			hue: true,
			interaction: {
				input: true,
				save: true
			}
		}
	});

	const bgPicker = Pickr.create({
		el: '#bgPicker',
		theme: 'monolith',
		default: bgInput.value,
		components: {
			preview: true,
			opacity: true,
			hue: true,
			interaction: {
				input: true,
				save: true
			}
		}
	});

	fgPicker.on('save', (color) => {
		fgInput.value = color.toHEXA().toString();
		fgPicker.hide();
	});

	bgPicker.on('save', (color) => {
		bgInput.value = color.toHEXA().toString();
		bgPicker.hide();
	});

});
