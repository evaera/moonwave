import clsx from "clsx"
import React from "react"
import styles from "./styles.module.css"

const BADGES = {
  Server: {
    color: "#00CC67",
    title: "This item only works when running on the server.",
    image: (
      <svg
        fill="#00CC67"
        xmlns="http://www.w3.org/2000/svg"
        version="1.1"
        x="0px"
        y="0px"
        viewBox="0 0 90 90"
        role="img"
        aria-label="This item only works when running on the server."
      >
        <polygon points="6.125,72.421 58.544,88.523 58.544,24.372 6.125,8.269 "></polygon>
        <polygon points="79.818,15.77 79.868,15.755 29.615,1.453 10.328,6.318 60.102,21.609 "></polygon>
        <path d="M61.643,78.738l22.232-6.582V17.801l-22.232,6.584V78.738z M73.811,69.257c-1.174,0-2.125-1.19-2.125-2.659  c0-1.468,0.951-2.658,2.125-2.658c1.172,0,2.123,1.19,2.123,2.658C75.934,68.066,74.982,69.257,73.811,69.257z M64.898,32.121  l16.336-4.41v7.117l-16.336,4.409V32.121z M64.898,41.292l16.336-4.41v7.116l-16.336,4.409V41.292z M64.898,50.463l16.336-4.411  v7.116l-16.336,4.411V50.463z"></path>
        <polygon points="83.875,74.893 61.643,81.477 61.643,88.547 83.875,81.963 "></polygon>
      </svg>
    ),
  },
  Client: {
    color: "#349AD5",
    title: "This item only works when running on the client.",
    image: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 337 304"
        fill="#349AD5"
        role="img"
        aria-label="This item only works when running on the client."
      >
        <path d="M0 201h337v36H0v-36zm0 0zM0 0v188h337V0H0zM218 287h30c12 0 12 17 0 17H88c-11 0-11-17 0-17h31l10-39h79l10 39z" />
      </svg>
    ),
  },
  Plugin: {
    color: "#f39c12",
    title: "This item only works when running in the context of a plugin.",
    image: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 100 100"
        fill="#f39c12"
        style={{ transform: "rotate(45deg)" }}
        role="img"
        aria-label="This item only works when running in the context of a plugin."
      >
        <path d="M74.388 29.812H25.612a2.152 2.152 0 0 0-2.152 2.152v5.315c0 1.188.963 2.152 2.152 2.152h2.87v8.314c0 9.634 6.333 17.792 15.063 20.533v5.052c0 .792.642 1.435 1.434 1.435h1.435v1.436c0 6.703 2.612 13.008 7.354 17.75.698.699 1.617 1.049 2.535 1.049s1.836-.35 2.536-1.051c1.4-1.4 1.4-3.672 0-5.072a17.81 17.81 0 0 1-5.252-12.678v-1.436h1.435c.792 0 1.434-.643 1.434-1.435v-5.052c8.731-2.743 15.063-10.898 15.063-20.533V39.43h2.869a2.152 2.152 0 0 0 2.152-2.152v-5.315a2.152 2.152 0 0 0-2.152-2.151zM41.392 8.586a3.586 3.586 0 0 0-7.173 0v18.649h7.173V8.586zM65.78 8.586a3.586 3.586 0 0 0-7.173 0v18.649h7.173V8.586z" />
      </svg>
    ),
  },
  Yields: {
    color: "#f1c40f",
    title:
      "This is a yielding function. When called, it will pause the Lua thread that called the function until a result is ready to be returned, without interrupting other scripts.",
    image: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 100 100"
        fill="#f1c40f"
        role="img"
        aria-label="This is a yielding function. When called, it will pause the Lua thread that called the function until a result is ready to be returned, without interrupting other scripts."
      >
        <path d="M96.4 11.9c-1.4-2.5-4-4-6.9-4H10.8c-2.9 0-5.4 1.5-6.9 4-1.4 2.5-1.4 5.5 0 7.9L43.3 88c1.4 2.5 4 4 6.9 4 2.9 0 5.4-1.5 6.9-4l39.3-68.2c1.4-2.4 1.4-5.4 0-7.9zM46.3 54.5c1.1-1.1 2.3-1.6 3.8-1.6s2.8.5 3.8 1.6c1.1 1.1 1.6 2.3 1.6 3.8s-.5 2.8-1.6 3.8c-1.1 1.1-2.3 1.6-3.8 1.6s-2.8-.5-3.8-1.6c-1.1-1.1-1.6-2.3-1.6-3.8.1-1.5.6-2.8 1.6-3.8zm-.5-4.9V21.8h8.8v27.8h-8.8z" />
      </svg>
    ),
  },
  Private: {
    color: "#9b59b6",
    title: "This item is only intended to be used by the module's authors.",
    image: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 100 100"
        fill="#9b59b6"
        role="img"
        aria-label="This item is only intended to be used by the module's authors."
      >
        <path d="m49.2 36 14.3 12.7.1-.7c0-6.7-6.1-12.1-13.6-12.1l-.8.1z" />
        <path d="M50 27.8c12.5 0 22.7 9 22.7 20.1 0 2.6-.6 5-1.6 7.3L84.4 67c6.9-5 12.3-11.6 15.6-19.1-7.9-17.6-27.3-30.1-50-30.1-6.4 0-12.5 1-18.1 2.8l9.8 8.6c2.6-.8 5.4-1.4 8.3-1.4zM4.5 16.9l10.4 9.2 2.1 1.8C9.5 33.1 3.5 40 0 48c7.9 17.7 27.3 30.2 50 30.2 7 0 13.8-1.2 19.9-3.4l1.9 1.7 13.3 11.7 5.8-5.1-80.6-71.3-5.8 5.1zm25.2 22.2 7 6.2c-.2.8-.3 1.8-.3 2.6C36.4 54.6 42.5 60 50 60c1 0 2-.2 3-.3l7 6.2c-3 1.3-6.4 2.1-10 2.1-12.5 0-22.7-9-22.7-20.1 0-3.1.9-6.1 2.4-8.8z" />
      </svg>
    ),
  },
  "Read Only": {
    color: "#e74c3c",
    title: "This item is read only and cannot be modified.",
    image: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="300px"
        width="300px"
        fill="#e74c3c"
        x="0px"
        y="0px"
        viewBox="0 0 96 96"
      >
        <path d="M70.354,15.662h-7.496H25.566c-3.282,0-5.19-0.776-5.728-1.169v-1.456c0.538-0.393,2.445-1.168,5.728-1.168h46.479V8H25.566  c-4.776,0-9.594,1.468-9.594,4.747v61.458C15.972,86.645,20.328,88,37.495,88h42.535V15.662H70.354z" />
      </svg>
    ),
  },
}

export default function Badge({ label }) {
  const { color, image, title } = BADGES[label] ?? {
    color: "red",
    image: <svg />,
    title: "",
  }

  return (
    <>
      <span
        className={clsx(styles.badge)}
        style={{
          color,
        }}
      >
        <span className={clsx(styles.badgeTooltip)}>{title}</span>
        <span className={clsx(styles.badgeToolTipTail)}>
          {image} {label}
        </span>
      </span>
    </>
  )
}
