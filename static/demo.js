const POLLING_INTERVAL = 500; // Polling interval in milliseconds

document.addEventListener("DOMContentLoaded", () => {
  console.log("🧠 DOM ready");
  const form = document.getElementById("waveformForm");
  const img = document.getElementById("preview");
  const submitButton = document.getElementById("submitButton");
  const statusDiv = document.getElementById("status");

  async function pollStatus(jobID) {
    let keepPolling = true;
    statusDiv.textContent = "Polling for job status...";

    while (keepPolling) {
      const res = await fetch(`/lissajous/status/${encodeURIComponent(jobID)}`);
      if (!res.ok) {
        statusDiv.textContent = "Failed to get status.";
        keepPolling = false; // Stop polling
      } else if (res.headers.get("Content-Type") === "application/json") {
        const data = await res.json();
        statusDiv.textContent = data.status;

        if (data.status === "error") {
          statusDiv.textContent = "Error: " + (data.message || "Unknown error");
          keepPolling = false; // Stop polling
        } else if (data.status === "done") {
          // Fetch the image
          const imgRes = await fetch(
            `/lissajous/result?id=${encodeURIComponent(jobID)}`
          );
          if (imgRes.ok) {
            const blob = await imgRes.blob();
            img.src = URL.createObjectURL(blob);
            img.classList.remove("invisible");
            statusDiv.textContent = "Waveform ready!";
          } else {
            statusDiv.textContent = "Failed to fetch image.";
          }

          keepPolling = false; // Stop polling
        }
      } else {
        statusDiv.textContent =
          "Unexpected response type: " + res.headers.get("Content-Type");
        keepPolling = false; // Stop polling
      }

      await new Promise((r) => setTimeout(r, POLLING_INTERVAL)); // Wait before polling again
    }
    submitButton.disabled = false;
    submitButton.classList.remove("opacity-50", "cursor-not-allowed");
  }

  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    img.classList.add("invisible");
    submitButton.disabled = true;
    submitButton.classList.add("opacity-50", "cursor-not-allowed");
    statusDiv.textContent = "Submitting job...";

    const fgHexInput = document.getElementById("fgColorHex").value.trim();
    const bgHexInput = document.getElementById("bgColorHex").value.trim();
    const hexPattern = /^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/;

    if (fgHexInput && hexPattern.test(fgHexInput)) {
      fgInput.value = fgHexInput;
    }
    if (bgHexInput && hexPattern.test(bgHexInput)) {
      bgInput.value = bgHexInput;
    }

    const data = new URLSearchParams(new FormData(form));

    try {
      const res = await fetch("/lissajous", {
        method: "POST",
        body: data,
		headers: {
			"Accept": "application/json",
			"Content-Type": "application/x-www-form-urlencoded" // Ensure correct content type
		}
      });
      if (!res.ok) {
        statusDiv.textContent = "Failed to start job.";
        submitButton.disabled = false;
        submitButton.classList.remove("opacity-50", "cursor-not-allowed");
        return;
      }
      const result = await res.json();
      const { jobID } = result;
      if (!jobID) {
        statusDiv.textContent = "Job ID not returned.";
        submitButton.disabled = false;
        submitButton.classList.remove("opacity-50", "cursor-not-allowed");
        return;
      } else {
        statusDiv.textContent = "Job started. Waiting for status...";
        pollStatus(jobID);
      }
    } catch (err) {
      statusDiv.textContent = "Error: " + err.message;
      submitButton.disabled = false;
      submitButton.classList.remove("opacity-50", "cursor-not-allowed");
    }
  });

  const fgInput = document.querySelector('input[name="fgColor"]');
  const bgInput = document.querySelector('input[name="bgColor"]');

  const fgPicker = Pickr.create({
    el: "#fgPicker",
    theme: "monolith",
    default: fgInput.value,
    components: {
      preview: true,
      opacity: true,
      hue: true,
      interaction: {
        input: true,
        save: true,
      },
    },
  });

  const bgPicker = Pickr.create({
    el: "#bgPicker",
    theme: "monolith",
    default: bgInput.value,
    components: {
      preview: true,
      opacity: true,
      hue: true,
      interaction: {
        input: true,
        save: true,
      },
    },
  });

  fgPicker.on("save", (color) => {
    const hex = color.toHEXA().toString();
    fgInput.value = hex;                 // hidden input
    document.getElementById("fgColorHex").value = hex;  // visible hex input
    fgPicker.hide();
  });

  bgPicker.on("save", (color) => {
    const hex = color.toHEXA().toString();
    bgInput.value = hex;                 // hidden input
    document.getElementById("bgColorHex").value = hex;  // visible hex input
    bgPicker.hide();	
  });
});
