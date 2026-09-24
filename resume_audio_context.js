(function() {
  const contexts = [];

  const OriginalAudioContext =
    window.AudioContext || window.webkitAudioContext;

  window.AudioContext = new Proxy(OriginalAudioContext, {
    construct(target, args) {
      const context = new target(...args);

      contexts.push(context);

      console.log("Captured AudioContext:", context.state);

      return context;
    }
  });

  function resumeAudio() {
    console.log("User interacted. Contexts:", contexts.length);

    for (const context of contexts) {
      console.log("Before resume:", context.state);

      if (context.state === "suspended") {
        context.resume().then(() => {
          console.log("After resume:", context.state);
        });
      }
    }
  }

  document.addEventListener("click", resumeAudio);
  document.addEventListener("keydown", resumeAudio);
  document.addEventListener("touchend", resumeAudio);
  document.addEventListener("pointerup", resumeAudio);
})();
