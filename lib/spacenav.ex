defmodule Spacenav do
  use Rustler, otp_app: :spacenav, crate: "spacenav"

  def listen(pid), do: :erlang.nif_error(:nif_not_loaded)
  def listen, do: listen(self())
end

defmodule Spacenav.ButtonEvent do
  defstruct [:bnum, :press]
end

defmodule Spacenav.MotionEvent do
  defstruct [:x, :y, :z, :rx, :ry, :rz, :period]
end
