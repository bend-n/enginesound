extends EngineNoise

@export var player: AudioStreamPlayer;

func _ready() -> void:
	set_process(false)
	player.play()
	make_engine()
	set_stream(player.get_stream_playback())
	set_process(true)

func _process(_d: float):
	update()

func _on_vol_value_changed(value: float) -> void:
	volume = value

func _on_rpm_value_changed(value: float) -> void:
	rpm = value
