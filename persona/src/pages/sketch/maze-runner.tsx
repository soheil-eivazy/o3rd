import dynamic from 'next/dynamic'
import { Maze, GraphSearchMethod } from "../../../sketches/pkg"
// import { memory } from "sketches/sketches_bg.wasm"
import p5Types from "p5"
// import { useContext } from "react"

// import { WASMContext } from "../../context/WASM"


const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)


let mazeWidth: number
let mazeHeight: number
let cellWidth: number
let cellHeight: number
let maze: Maze
let plane: Uint8Array

const mazeRunner = () => {

	const setup = (p5: p5Types, canvasParentRef: Element) => {
		p5.frameRate(30);
		let size = window.innerWidth > window.innerHeight ? window.innerHeight : window.innerWidth
		size = size - (size % 100)
		p5.createCanvas(size, size).parent(canvasParentRef)

		mazeWidth = 400
		mazeHeight = 400
		cellWidth = size / mazeWidth
		cellHeight = size / mazeHeight

		maze = Maze.new(mazeWidth, mazeHeight, GraphSearchMethod.BreadthFirst)
		plane = maze.flat_plane()
		draw_plane(p5)
		// p5.colorMode(p5.HSB)
		
	}

	const draw = (p5: p5Types) => {
		// p5.print(c++)
		p5.print("draw!!!")
		let step: Uint16Array = maze.run()
		if (step.length == 0) {
			p5.noLoop()
			return 
		}

		
		if (step[0] == 1) {
			draw_plane(p5)
			draw_solved(p5, step)
			
			p5.noLoop()
		} else {
			draw_steps(p5, step)
		}
		
		
		

		

		// const cellsPtr = maze.cells();
	}

	const draw_steps = (p5: p5Types, step: Uint16Array) => {
		p5.stroke(255, 0, 0)
		p5.strokeWeight(cellHeight / 2)
		for (let i = 1; i < step.length;) {
			let x = step[i];
			let y = step[++i];
			let x1 = step[++i];
			let y1 = step[++i];
			
			p5.line(
				(x*cellWidth)  + (cellWidth  ), 
				(y*cellHeight) + (cellHeight / 2), 
				(x1*cellWidth) + (cellWidth  ), 
				(y1*cellHeight)+ (cellHeight / 2)
			)

			i++
	}

	}

	const draw_solved = (p5: p5Types, step: Uint16Array) => {
		p5.stroke(255, 0, 0)
		p5.strokeWeight(cellHeight / 2)
		for (let i = 1; i < step.length;) {
			let x = step[i];
			let y = step[++i];
			let x1 = step[++i];
			let y1 = step[i+1];
			
			p5.line(
				(x*cellWidth)  + (cellWidth  ), 
				(y*cellHeight) + (cellHeight / 2), 
				(x1*cellWidth) + (cellWidth  ), 
				(y1*cellHeight)+ (cellHeight / 2)
			)
		}

		p5.print(step)
	}

	const draw_plane = (p5: p5Types) => {
		p5.background(0)
		p5.stroke(255, 255, 255)
		p5.strokeWeight(0)
		for (let row = 0; row < mazeHeight; row++) {
			for (let col = 0; col < mazeWidth; col++) {

				const idx = getIndex(row, col);
				if (plane[idx] ==  1) {
					p5.fill(255, 255, 255)
				} else {
					p5.fill(0, 0, 0)
				}

				p5.rect((col * cellWidth) + (cellWidth / 2), row * cellHeight, cellWidth, cellHeight)
			}
		}

		let fl = maze.first_and_last()
		p5.fill(255, 255, 0)
		p5.rect((fl[0] * cellWidth) + (cellWidth / 2), fl[1] * cellHeight, cellWidth, cellHeight)
		p5.fill(255, 0, 2550)
		p5.rect((fl[2] * cellWidth) + (cellWidth / 2), fl[3] * cellHeight, cellWidth, cellHeight)
	}
	// const keyPressed = (p5: p5Types) => {}

	return <Sketch setup={setup} draw={draw} /> // keyPressed={keyPressed}  />
}


const getIndex = (row: number, column: number) => {
    return row * mazeWidth + column;
};


export default mazeRunner



// const setup = (p5: p5Types, canvasParentRef: Element) => {
// 	p5.frameRate(30);
// 	let size = window.innerWidth > window.innerHeight ? window.innerHeight : window.innerWidth
// 	size = size - (size % 100)
// 	p5.createCanvas(size, size).parent(canvasParentRef)

// 	mazeWidth = 50;
// 	mazeHeight = 50;

// 	cellWidth = Math.floor(size / mazeWidth)
// 	cellHeight = Math.floor(size / mazeHeight)

// 	maze = BreadthFirst.new(mazeWidth, mazeHeight)
// 	p5.colorMode(p5.HSB)
// 	p5.background(0)
// };

// const draw = (p5: p5Types) => {
// 	p5.print(c++)
// 	maze.run(c)

// 	const cellsPtr = maze.cells();
// 	const cells = new Uint8Array(memory.buffer, cellsPtr, mazeWidth * mazeHeight);


// 	for (let row = 0; row < mazeHeight; row++) {
// 		for (let col = 0; col < mazeWidth; col++) {
// 			const idx = getIndex(row, col);

// 			switch(cells[idx]) {
// 				case 0:
// 					p5.fill(100, 0, 0)
// 					break;
// 				case c:
// 					p5.fill(0, 100, 100)
// 					break;

// 				default:
// 					p5.fill(100, 0, 100)
// 			}

// 			p5.rect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
// 		}
// 	}

// 	if (maze.is_done()) p5.noLoop()
// };

// const keyPressed = (p5: p5Types) => {
// 	switch(p5.key) {
// 		case 's':
// 			p5.loop()
// 			break;
// 		case 'Q':
// 			// code block
// 			break;
// 		case 'R':
// 			// code block
// 			break;

// 		case 'f':
// 			p5.noLoop()
// 			break;
// 		default:
// 			// code block
// 	}
// }

// return <Sketch setup={setup} draw={draw} keyPressed={keyPressed}  />