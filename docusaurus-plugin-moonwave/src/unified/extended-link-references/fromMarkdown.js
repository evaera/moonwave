export default function fromMarkdown() {
  function enter(token) {
    this.enter(
      {
        type: "linkReference",
        referenceType: "shortcut",
      },
      token
    )
  }

  function exit(token) {
    this.exit(token)
  }

  function exitLabel(token) {
    const label = this.sliceSerialize(token)
    const node = this.stack[this.stack.length - 1]
    node.identifier = label
    node.label = label
    node.children = [{ type: "text", value: label }]
  }

  return {
    enter: {
      extendedLinkReference: enter,
    },
    exit: {
      extendedLinkReference: exit,
      extendedLinkReferenceLabel: exitLabel,
    },
  }
}
