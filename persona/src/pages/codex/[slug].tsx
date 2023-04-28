import Link from "next/link"


export default function Home() {
    return (
      <>
        <p>list of sketches</p>
        <ul>
            <li>
                <Link href="/sketch/maze-runner">maze-runner</Link>
            </li>
        </ul>
      </>
    )
  }
  