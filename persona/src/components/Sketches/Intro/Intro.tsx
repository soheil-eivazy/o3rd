import dynamic from 'next/dynamic'
import p5Types from "p5"
import IntroFrag from './intro.frag'
import IntroVert from './intro.vert'

const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)


const IntroSketch = () => {

    let screen: p5Types.Graphics
    let shader: p5Types.Shader
    let font: p5Types.Font

    const preload = (p5: p5Types) => {
        font = p5.loadFont('/IBM 3270 Nerd Font Complete.ttf')
    }

	const setup = (p5: p5Types, canvasParentRef: Element) => {
		p5.createCanvas(window.innerWidth, window.innerHeight, p5.WEBGL).parent(canvasParentRef)
        shader = p5.createShader(IntroVert, IntroFrag);

        screen = p5.createGraphics(p5.width, p5.height);
  
        setupScreen(p5, window.innerWidth, window.innerHeight)

        p5.shader(shader);
        // p5.createCanvas(window.innerWidth, window.innerHeight, p5.WEBGL);

        // // create and initialize the shader
        // shader = p5.createShader(IntroVert, IntroFrag);
        // p5.shader(shader);
        // p5.noStroke();

        // // 'p' is the center point of the Mandelbrot image
        // shader.setUniform('p', [-0.74364388703, 0.13182590421]);
        // p5.describe('zooming Mandelbrot set. a colorful, infinitely detailed fractal.');
	}

    const setupScreen = (p5: p5Types, width: number, height: number) => {
        screen.background(0);
        screen.stroke(255);
        screen.strokeWeight(5);
        screen.textSize(50)
        screen.fill(255, 255, 255)
        screen.textFont(font)
        screen.text(`Soheil Eivazy, 

developer and 

some other things`, 0, 0, width, height)
    }

	const draw = (p5) => {

        // 'r' is the size of the image in Mandelbrot-space
        // shader.setUniform('r', 1.5 * p5.exp(-6.5 * (1 + p5.sin(p5.millis() / 2000))));
        // p5.quad(-1, -1, 1, -1, 1, 1, -1, 1);

		if(p5.mouseIsPressed) {
            screen.line(p5.mouseX, p5.mouseY, p5.pmouseX, p5.pmouseY);
        }

        drawScreen(p5);
	}

    function drawScreen(p5: p5Types) {
        shader.setUniform('texture', screen);
        shader.setUniform('noise', getNoiseValue(p5));

        p5.rect(-p5.width/2, -p5.height/2, p5.width, p5.height);
    }
      
    function getNoiseValue(p5: p5Types) { 
        let v = p5.noise(p5.millis()/100);
        const cutOff = 0.5;
        
        if(v < cutOff) {
            return 0;
        }
        
        v = p5.pow((v-cutOff) * 1/(1-cutOff), 2);
        
        return v;
    }


	return <Sketch className={"intro-sketch"} setup={setup} draw={draw} preload={preload}/> // keyPressed={keyPressed}  />
}


export default IntroSketch
