extends AudioStreamPlayer

@onready var strem: EngineStream = stream

func _ready() -> void:
	set_process(false)
	play()
	strem.set_stream(get_stream_playback())
	set_process(true)

func _process(_d: float):
	strem.update()

func _on_vol_value_changed(value: float) -> void:
	volume_db = linear_to_db(value)

func _on_rpm_value_changed(value: float) -> void:
	strem.engine_rpm = value
