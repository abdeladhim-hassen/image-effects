async function init() {
    let rustapp = null;
    try {
        // Import the Rust module using await since import returns a promise.
        rustapp = await import('../pkg');
    } catch (e) {
        console.error(e);
        return;
    }

    console.log(rustapp);

    const input = document.getElementById('upload');
    const fileReader = new FileReader();

    fileReader.onloadend = () => {
        let base64 = fileReader.result.replace(
            /^data:image\/(png|jpeg|jpg);base64,/, ''
        );

        // Ensure that the grayscale function exists in the imported module.
        if (rustapp.grayscale) {
          let img_data_url =   rustapp.grayscale(base64);
          document.getElementById('new-img').setAttribute(
            'src',img_data_url
          )
        } else {
            console.error('grayscale function not found in Rust module');
        }
    };

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0]);
    });
}

init();
