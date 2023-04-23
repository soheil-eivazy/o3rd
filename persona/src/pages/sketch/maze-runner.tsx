import dynamic from 'next/dynamic'
import { BreadthFirst, DepthFirst } from "sketches"
import { memory } from "sketches/sketches_bg.wasm"
import p5Types from "p5"
import { useContext } from "react"

import { WASMContext } from "../../context/WASM"


const Sketch = dynamic(
    () => import('react-p5'),
    { ssr: false }
)


let maze: BreadthFirst
let mazeWidth: number
let mazeHeight: number
let cellWidth: number
let cellHeight: number
let c = 1


const mazeRunner = () => {

	const ctx = useContext(WASMContext)

	if (!ctx.wasm) {
		return <>...</>
	}

    const setup = (p5: p5Types, canvasParentRef: Element) => {
		p5.frameRate(30);
		let size = window.innerWidth > window.innerHeight ? window.innerHeight : window.innerWidth
		size = size - (size % 100)
		p5.createCanvas(size, size).parent(canvasParentRef)

		mazeWidth = 50;
		mazeHeight = 50;

		cellWidth = Math.floor(size / mazeWidth)
		cellHeight = Math.floor(size / mazeHeight)

		maze = BreadthFirst.new(mazeWidth, mazeHeight)
		p5.colorMode(p5.HSB)
		p5.background(0)
	};

	const draw = (p5: p5Types) => {
		p5.print(c++)
		maze.run(c)

		const cellsPtr = maze.cells();
		const cells = new Uint8Array(memory.buffer, cellsPtr, mazeWidth * mazeHeight);


		for (let row = 0; row < mazeHeight; row++) {
			for (let col = 0; col < mazeWidth; col++) {
				const idx = getIndex(row, col);

				switch(cells[idx]) {
					case 0:
						p5.fill(100, 0, 0)
						break;
					case c:
						p5.fill(0, 100, 100)
						break;

					default:
						p5.fill(100, 0, 100)
				}

				p5.rect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
			}
		}

		if (maze.is_done()) p5.noLoop()
	};

	const keyPressed = (p5: p5Types) => {
		switch(p5.key) {
			case 's':
				p5.loop()
				break;
			case 'Q':
				// code block
				break;
			case 'R':
				// code block
				break;

			case 'f':
				p5.noLoop()
				break;
			default:
				// code block
		}
	}

	return <Sketch setup={setup} draw={draw} keyPressed={keyPressed}  />
}


const getIndex = (row: number, column: number) => {
    return row * mazeWidth + column;
};


export default mazeRunner