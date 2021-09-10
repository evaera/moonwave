import clsx from "clsx"
import React from "react"
import styles from "./styles.module.css"

const colors = [
  "#dc0073",
  "#ff5252",
  "#ff793f",
  "#2b8ab6",
  "#00a1e4",
  "#218c74",
  "#79b473",
  "#70cc00",
  "#925FB9",
]

const getTagColor = (text) => {
  let sum = 0

  for (let i = 0; i < text.length; i++) {
    sum += text.charCodeAt(i)
  }

  return colors[sum % colors.length]
}

export default function Tag({ label }) {
  const color = getTagColor(label)

  return (
    <>
      <span
        className={clsx(styles.docTag)}
        style={{
          borderColor: color,
          color: color,
        }}
      >
        {label}
      </span>
    </>
  )
}
