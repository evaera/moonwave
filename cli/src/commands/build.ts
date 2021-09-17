import { Args } from "../argv"
import { prepareProject } from "../prepareProject"

export default function buildCommand(args: Args) {
  prepareProject(process.cwd(), args)

  console.log("built different")
}
