import React from "react"
import styles from "./styles.module.css"

const DEPTH_CLASSES = [
  styles.purple,
  styles.green,
  styles.yellow,
  styles.cyan,
  styles.orange,
  styles.red,
]

// Operator
export const Op = ({ children, depth }) => (
  <code
    className={
      depth !== undefined
        ? DEPTH_CLASSES[(depth - 1) % DEPTH_CLASSES.length]
        : styles.op
    }
  >
    {children}
  </code>
)

// Prominent Operator
export const PrOp = ({ children }) => <code>{children}</code>
