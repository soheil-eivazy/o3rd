import { useState, createContext } from 'react'
import type { ReactNode } from 'react'
import { useMountEffectOnce } from '../hooks/useMountEffectOnce'

const initial: IWASMContext = {}

export const WASMContext = createContext(initial)

export const WASMContextProvider: React.FC<WASMContextProviderProps> = ({
  children
}) => {
  const [state, setState] = useState<IWASMContext>(initial)

  // This has to run only once: https://github.com/rustwasm/wasm-bindgen/issues/3153
  // Though, in development React renders twice when Strict Mode is enabled: https://reactjs.org/docs/strict-mode.html
  // That's why it must be limited to a single mount run
  useMountEffectOnce(() => {
    (async() => {
      const wasm = await import("sketches");
      const memory = await import("sketches/sketches_bg.wasm");
      const p5 = await import('react-p5');
      await wasm.default();
      setState({ wasm, memory, p5 });
    })()
  })

  return (
    <WASMContext.Provider value={state}>
      {children}
    </WASMContext.Provider>
  )
}

interface IWASMContext {
  wasm?: typeof import('sketches')
  memory?: typeof import('sketches/sketches_bg.wasm')
  p5?: typeof import('react-p5')
}

interface WASMContextProviderProps {
  children: ReactNode
}
