import {Pane} from 'tweakpane';


async function main() {

  // Load the wasm lib
  const lib = await import("../pkg/index.js").catch(console.error);
  const canvasSize = 512;
  const processor = new lib.ImageProcessor(canvasSize, canvasSize);

  const PARAMS = {
    sortPixels: true,
    vertical: false,
    threshVal: 150,
    slidePixels: true,
    maxSlide: 50,
    numSlides: 10,
    shuffleAreas: true,
    numShuffles: 5,
    imageHint: "waves"
  }

  const updateImageFunc = (ev) => {
    updateImage();
    if (ev.last) {
    }
  }
  
  // ===== UI ====================================

  const pane = new Pane({
    title: 'Pixel Glitch',
    expanded: true,
    container: document.getElementById('tweakpane-container'),
  });
  pane.element.classList.add('tweakpane-panel');
  
  var sort = pane.addFolder({title: 'Pixel Sort'});
  sort.addInput(PARAMS, 'sortPixels').on('change', updateImageFunc);
  sort.addInput(PARAMS, 'vertical').on('change', updateImageFunc);
  sort.addInput(PARAMS, 'threshVal', {min: 0, max: 255, step: 1}).on('change', updateImageFunc);

  var slide = pane.addFolder({title: 'Pixel Slide'});
  slide.addInput(PARAMS, 'slidePixels').on('change', updateImageFunc);
  slide.addInput(PARAMS, 'numSlides', {min: 0, max: 50, step: 1}).on('change', updateImageFunc);
  slide.addInput(PARAMS, 'maxSlide', {min: 0, max: 400, step: 1}).on('change', updateImageFunc);

  var area = pane.addFolder({title: 'Area Glitch'});
  area.addInput(PARAMS, 'shuffleAreas').on('change', updateImageFunc);
  area.addInput(PARAMS, 'numShuffles', {min: 0, max: 20, step: 1}).on('change', updateImageFunc);
  area.addInput(PARAMS, 'imageHint');

  pane.addButton({title: 'New Unsplash Image'}).on('click', (ev) => {
    loadNewUnsplashImage();
  });
  pane.addButton({title: 'Regenerate'}).on('click', (ev) => {
    updateImage();
  });
  pane.addButton({title: 'Download'}).on('click', (ev) => {
    saveImage();
  });

  // Initialise canvas
  const canvas = document.getElementById("my-canvas");
  const ctx = canvas.getContext("2d");
  canvas.width = canvasSize;
  canvas.height = canvasSize;

  // Add drag and drop image load
  canvas.addEventListener("dragover", (ev) => {
    ev.preventDefault();
  }, false);

  canvas.addEventListener("drop", (ev) => {
    const files = ev.dataTransfer.files;
    if (files.length > 0) {
      const file = files[0];
      if (typeof FileReader !== "undefined" && file.type.indexOf("image") != -1) {
        const reader = new FileReader();
        reader.onload = (ev) => {
          loadImage(ev.target.result);
        };
        reader.readAsDataURL(file);
      }
    }
    ev.preventDefault();
  }, false);
  
  
  let image;
  let image_data;
  let imageIsLoaded = false;
  loadNewUnsplashImage();

  function loadNewUnsplashImage() {
    let imageUrl = "https://source.unsplash.com/512x512/?" + PARAMS.imageHint;
    imageUrl += ",sig=" + Math.floor(Math.random() * 1000);
    loadImage(imageUrl);
  }

  function loadImage(src) {
    imageIsLoaded = false;
    image = new Image();
    image.display = "none";
    image.crossOrigin = "Anonymous"
    image.onload = () => {
      console.log("loaded new image");
      imageIsLoaded = true;
      ctx.drawImage(image, 0, 0, canvasSize, canvasSize);
      image_data = ctx.getImageData(0, 0, canvas.width, canvas.height);

      processor.load_base_image(canvasSize, canvasSize, image_data.data);
      updateImage();
    }
    image.src = src;
  }

  function updateImage() {

    if (!imageIsLoaded) {
      console.error("Image is not loaded yet");
      return;
    }

    processor.reset_image();

    if (PARAMS.slidePixels) {
      processor.process_rect_slide(PARAMS.numSlides, PARAMS.maxSlide);
    }
    if (PARAMS.sortPixels) {
      processor.process_pixel_sort(PARAMS.threshVal, PARAMS.vertical);
    }
    if (PARAMS.shuffleAreas) {
      processor.process_area_shuffle(PARAMS.numShuffles);
    }

    const newData = processor.get_pixel_data();

    const clampedData = new Uint8ClampedArray(newData.buffer);
    const imgData = new ImageData(clampedData, canvas.width);

    ctx.putImageData(imgData, 0, 0);
  }

  function saveImage() {
    var anchor = document.createElement("a");
    anchor.href = canvas.toDataURL("image/png");
    anchor.download = "Glitch.png";
    anchor.click();
  }
}



main();
