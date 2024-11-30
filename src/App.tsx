import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const handleAddBorder = () => invoke("add_border");

  const handleRemoveBorder = () => invoke("remove_border");

  const handleChangeBorderColor = () =>
    invoke("change_border_color", {
      color: [255, 0, 0, 255],
    });

  return (
    <main className="container">
      <h1>Border demo</h1>

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          gap: "6px",
          justifyContent: "center",
        }}
      >
        <button type="button" onClick={handleAddBorder}>
          Add border
        </button>

        <button type="button" onClick={handleRemoveBorder}>
          Remove border
        </button>

        <button type="button" onClick={handleChangeBorderColor}>
          Change border border
        </button>
      </div>
    </main>
  );
}

export default App;
