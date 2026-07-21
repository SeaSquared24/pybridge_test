import tkinter as tk

class Window:
    def __init__(self):
        self.run = True
        self.root = tk.Tk()
        self.root.title("Test Window")
        self.root.geometry("600x600")
        self.root.resizable(True, True)

        self.button = tk.Button(
            self.root, 
            text="Click Me", 
            command=self.on_button_click
        )
        self.button.pack()

        self.root.protocol("WM_DELETE_WINDOW", self.handler)

    def handler(self):
        self.run = False
        self.root.destroy()

    def loop(self):
        if self.run:
            # Update logic here

            # after stays at the end of the loop to ensure it runs continuously.
            self.root.after(16, self.loop)

    def on_button_click(self):
        # Going to call a Rust function.

def main():
    window = Window()
    window.loop()
    window.root.mainloop()

if __name__ == "__main__":
    main()