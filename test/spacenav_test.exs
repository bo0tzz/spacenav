defmodule SpacenavTest do
  use ExUnit.Case
  doctest Spacenav

  test "greets the world" do
    assert Spacenav.hello() == :world
  end
end
