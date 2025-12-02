import React, {useState} from "react";

interface WasmCardProps {
  title: string;
  description: string;
  part1Fn: (text: string) => number | bigint;
  part2Fn: (text: string) => number | bigint;
}

export default function DayCard({
  title,
  description,
  part1Fn,
  part2Fn
}: WasmCardProps) {
  const [input, setInput] = useState("");
  const [output1, setOutput1] = useState("");
  const [output2, setOutput2] = useState("");

  function handleChange(e: React.ChangeEvent<HTMLTextAreaElement>) {
    const value = e.target.value;
    setInput(value);
    setOutput1(String(part1Fn(value)));
    setOutput2(String(part2Fn(value)));
  }

  return (
    <div className="container">
      <article className="message">
        <div className="message-header">
          <div className="box">
            <h1 className="title is-1">{title}</h1>
          </div>
          <div className="box">
            <button className="button">
              <a href="../index.html">Go Home</a>
            </button>
          </div>
        </div>

        <div className="message-body">
          <div className="notification">
            <p>{description}</p>
          </div>

          <div className="card">
            <div className="card-content">
              <div className="content">
                <label>Enter Text:</label>
                <textarea
                  className="textarea"
                  value={input}
                  onChange={handleChange}
                ></textarea>
              </div>
            </div>

            <footer className="card-footer">
              <span className="container">
                <div className="box">
                  <span>Output Part 1: </span>
                  <strong>{output1}</strong>
                </div>
              </span>

              <span className="container">
                <div className="box">
                  <span>Output Part 2: </span>
                  <strong>{output2}</strong>
                </div>
              </span>
            </footer>
          </div>
        </div>
      </article>
    </div>
  );
}