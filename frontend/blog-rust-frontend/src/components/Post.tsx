import React, { useRef, useState } from "react";
import { ListOfPost } from "./ListOfPost";

export const Post = () => {
  const fromRef = useRef<HTMLFormElement>(null);
  const [changes, setChanges] = useState(false);
  const handlerSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!fromRef.current) {
      return;
    }

    const formData = new FormData(fromRef.current);
    const name = formData.get("name") as string;
    const description = formData.get("description") as string;
    const request = {
      id: Math.random().toString(),
      name,
      description,
    };
    const reponse = await fetch("http://127.0.0.1:8080/", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(request),
    });
    console.log(reponse);
    setChanges(!changes);
    fromRef.current.reset();
  };
  return (
    <div>
      <form
        className="flex  flex-col w-2/12 gap-2"
        onSubmit={handlerSubmit}
        ref={fromRef}
      >
        <input name="name" />
        <input name="description" />
        <button className="text-white bg-blue-500 rounded-md" type="submit">
          Posterar
        </button>
      </form>
      {/* <ListOfPost changes={changes} /> */}
    </div>
  );
};
